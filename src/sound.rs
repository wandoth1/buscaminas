use std::f32::consts::PI;
use std::sync::mpsc::{Receiver, Sender, channel};

use macroquad::audio::{
    PlaySoundParams, Sound, load_sound_from_bytes, play_sound, play_sound_once, stop_sound,
};

use crate::music::MusicTrack;
use crate::synth::{SAMPLE_RATE, make_wav};

pub struct SoundManager {
    pub enabled: bool,
    sound_click: Option<Sound>,
    sound_flag: Option<Sound>,
    sound_unflag: Option<Sound>,
    sound_chord: Option<Sound>,
    sound_lose: Option<Sound>,
    sound_win: Option<Sound>,
}

impl SoundManager {
    pub async fn new() -> Self {
        let sample_rate = SAMPLE_RATE;

        let click_dur = 0.035;
        let n = (sample_rate as f32 * click_dur) as usize;
        let mut click_samples = Vec::with_capacity(n);
        for i in 0..n {
            let t = i as f32 / sample_rate as f32;
            let decay = (-90.0 * t).exp();
            let freq = 900.0 - 300.0 * (t / click_dur);
            click_samples.push((2.0 * PI * freq * t).sin() * decay * 0.7);
        }
        let sound_click_raw = make_wav(&click_samples, sample_rate);

        let flag_dur = 0.050;
        let n = (sample_rate as f32 * flag_dur) as usize;
        let mut flag_samples = Vec::with_capacity(n);
        for i in 0..n {
            let t = i as f32 / sample_rate as f32;
            let decay = (-40.0 * t).exp();
            let freq = 450.0 + 650.0 * (t / flag_dur);
            flag_samples.push((2.0 * PI * freq * t).sin() * decay * 0.75);
        }
        let sound_flag_raw = make_wav(&flag_samples, sample_rate);

        let unflag_dur = 0.045;
        let n = (sample_rate as f32 * unflag_dur) as usize;
        let mut unflag_samples = Vec::with_capacity(n);
        for i in 0..n {
            let t = i as f32 / sample_rate as f32;
            let decay = (-50.0 * t).exp();
            let freq = 800.0 - 400.0 * (t / unflag_dur);
            unflag_samples.push((2.0 * PI * freq * t).sin() * decay * 0.6);
        }
        let sound_unflag_raw = make_wav(&unflag_samples, sample_rate);

        let chord_dur = 0.075;
        let n = (sample_rate as f32 * chord_dur) as usize;
        let mut chord_samples = Vec::with_capacity(n);
        for i in 0..n {
            let t = i as f32 / sample_rate as f32;
            let d1 = (-70.0 * t).exp();
            let d2 = if t >= 0.03 {
                (-70.0 * (t - 0.03)).exp()
            } else {
                0.0
            };
            let val = (2.0 * PI * 650.0 * t).sin() * d1 + (2.0 * PI * 1050.0 * t).sin() * d2;
            chord_samples.push(val * 0.5);
        }
        let sound_chord_raw = make_wav(&chord_samples, sample_rate);

        let lose_dur = 0.65;
        let n = (sample_rate as f32 * lose_dur) as usize;
        let mut lose_samples = Vec::with_capacity(n);
        let mut seed: u32 = 123456789;
        for i in 0..n {
            let t = i as f32 / sample_rate as f32;
            let decay = (-5.5 * t).exp();
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let noise = ((seed as f32 / u32::MAX as f32) * 2.0 - 1.0) * 0.65;
            let rumble1 = (2.0 * PI * (80.0 - 40.0 * t) * t).sin() * 0.5;
            let rumble2 = (2.0 * PI * 45.0 * t).sin() * 0.35;
            lose_samples.push((noise + rumble1 + rumble2) * decay * 0.8);
        }
        let sound_lose_raw = make_wav(&lose_samples, sample_rate);

        let freqs = [523.25, 659.25, 783.99, 1046.50];
        let note_dur = 0.12;
        let total_dur = note_dur * freqs.len() as f32 + 0.35;
        let n = (sample_rate as f32 * total_dur) as usize;
        let mut win_samples = vec![0.0f32; n];
        for (idx, &freq) in freqs.iter().enumerate() {
            let start_i = (idx as f32 * note_dur * sample_rate as f32) as usize;
            let chord_len = (sample_rate as f32 * 0.45) as usize;
            for j in 0..chord_len {
                if start_i + j < n {
                    let t = j as f32 / sample_rate as f32;
                    let env = (-5.0 * t).exp();
                    let wave_val = (2.0 * PI * freq * t).sin() * 0.7
                        + (2.0 * PI * freq * 2.0 * t).sin() * 0.2
                        + (2.0 * PI * freq * 3.0 * t).sin() * 0.1;
                    win_samples[start_i + j] += wave_val * env * 0.45;
                }
            }
        }
        let sound_win_raw = make_wav(&win_samples, sample_rate);

        let sound_click = load_sound_from_bytes(&sound_click_raw).await.ok();
        let sound_flag = load_sound_from_bytes(&sound_flag_raw).await.ok();
        let sound_unflag = load_sound_from_bytes(&sound_unflag_raw).await.ok();
        let sound_chord = load_sound_from_bytes(&sound_chord_raw).await.ok();
        let sound_lose = load_sound_from_bytes(&sound_lose_raw).await.ok();
        let sound_win = load_sound_from_bytes(&sound_win_raw).await.ok();
        let enabled = sound_click.is_some()
            || sound_flag.is_some()
            || sound_unflag.is_some()
            || sound_chord.is_some()
            || sound_lose.is_some()
            || sound_win.is_some();

        Self {
            enabled,
            sound_click,
            sound_flag,
            sound_unflag,
            sound_chord,
            sound_lose,
            sound_win,
        }
    }

    fn play_sound(&self, sound: &Option<Sound>) {
        if self.enabled
            && let Some(sound) = sound
        {
            play_sound_once(sound);
        }
    }

    pub fn play_click(&self) {
        self.play_sound(&self.sound_click);
    }
    pub fn play_flag(&self) {
        self.play_sound(&self.sound_flag);
    }
    pub fn play_unflag(&self) {
        self.play_sound(&self.sound_unflag);
    }
    pub fn play_chord(&self) {
        self.play_sound(&self.sound_chord);
    }
    pub fn play_lose(&self) {
        self.play_sound(&self.sound_lose);
    }
    pub fn play_win(&self) {
        self.play_sound(&self.sound_win);
    }

    pub fn toggle(&mut self) -> bool {
        self.enabled = !self.enabled;
        self.enabled
    }
}

/// Volumen de la música, muy por debajo de los efectos para no taparlos.
const MUSIC_VOLUME: f32 = 0.32;

/// Tema con el que arranca el juego. `None` dejaría el arranque en silencio.
const DEFAULT_TRACK: Option<MusicTrack> = Some(MusicTrack::Relax);

/// Reproductor de los temas de fondo.
///
/// Generar un tema cuesta unos 450 ms, demasiado para el hilo del juego: la
/// ventana se quedaría congelada media pantalla al activar la música. Cada tema
/// se genera en un hilo aparte la primera vez que se pide y se recoge en
/// [`MusicManager::poll`], así que el arranque no paga nada y no hay tirón.
pub struct MusicManager {
    tx: Sender<(MusicTrack, Vec<u8>)>,
    rx: Receiver<(MusicTrack, Vec<u8>)>,
    relax: Option<Sound>,
    focus: Option<Sound>,
    /// Temas cuya generación ya se ha lanzado, para no lanzarla dos veces.
    requested: Vec<MusicTrack>,
    /// Tema que el jugador quiere oír.
    wanted: Option<MusicTrack>,
    /// Tema que está sonando de verdad. Puede ir por detrás de `wanted`
    /// mientras el hilo termina de generar.
    active: Option<MusicTrack>,
}

impl MusicManager {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        let mut manager = Self {
            tx,
            rx,
            relax: None,
            focus: None,
            requested: Vec::new(),
            wanted: None,
            active: None,
        };
        // Pide el tema por defecto ya: la generación se va haciendo en su hilo
        // mientras el juego arranca, y `poll` lo engancha en cuanto esté.
        manager.select(DEFAULT_TRACK);
        manager
    }

    /// Tema seleccionado, sonando o a punto de sonar.
    pub fn selected(&self) -> Option<MusicTrack> {
        self.wanted
    }

    /// Pide un tema concreto, o silencio con `None`.
    pub fn select(&mut self, track: Option<MusicTrack>) {
        self.wanted = track;
        if let Some(track) = track {
            self.request(track);
        }
        self.apply();
    }

    /// Recoge los temas que el hilo haya terminado y arranca el pendiente.
    /// Hay que llamarlo una vez por frame.
    pub async fn poll(&mut self) {
        while let Ok((track, wav)) = self.rx.try_recv() {
            // En nativo esto no cede frames: el await solo espera de verdad en
            // wasm, donde la descodificación no es inmediata.
            let sound = load_sound_from_bytes(&wav).await.ok();
            match track {
                MusicTrack::Relax => self.relax = sound,
                MusicTrack::Focus => self.focus = sound,
            }
        }
        self.apply();
    }

    fn sound_of(&self, track: MusicTrack) -> Option<&Sound> {
        match track {
            MusicTrack::Relax => self.relax.as_ref(),
            MusicTrack::Focus => self.focus.as_ref(),
        }
    }

    fn request(&mut self, track: MusicTrack) {
        if self.sound_of(track).is_some() || self.requested.contains(&track) {
            return;
        }
        self.requested.push(track);
        let tx = self.tx.clone();
        // Si el receptor ya no existe el envío falla sin más: el juego se está
        // cerrando y la música ya no importa.
        std::thread::spawn(move || {
            let _ = tx.send((track, track.wav()));
        });
    }

    fn apply(&mut self) {
        if self.active == self.wanted {
            return;
        }
        if let Some(active) = self.active
            && let Some(sound) = self.sound_of(active)
        {
            stop_sound(sound);
        }
        self.active = None;

        if let Some(wanted) = self.wanted
            && let Some(sound) = self.sound_of(wanted)
        {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: true,
                    volume: MUSIC_VOLUME,
                },
            );
            self.active = Some(wanted);
        }
    }
}

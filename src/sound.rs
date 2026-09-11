use std::f32::consts::PI;
use macroquad::audio::{load_sound_from_bytes, play_sound_once, Sound};

pub struct SoundManager {
    pub enabled: bool,
    sound_click: Sound,
    sound_flag: Sound,
    sound_unflag: Sound,
    sound_chord: Sound,
    sound_lose: Sound,
    sound_win: Sound,
}

impl SoundManager {
    pub async fn new() -> Self {
        let sample_rate = 22050;

        // 1. Click (35ms seco)
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

        // 2. Flag (50ms pop ascendente)
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

        // 3. Unflag (45ms pop descendente)
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

        // 4. Chord (75ms doble chasquido)
        let chord_dur = 0.075;
        let n = (sample_rate as f32 * chord_dur) as usize;
        let mut chord_samples = Vec::with_capacity(n);
        for i in 0..n {
            let t = i as f32 / sample_rate as f32;
            let d1 = (-70.0 * t).exp();
            let d2 = if t >= 0.03 { (-70.0 * (t - 0.03)).exp() } else { 0.0 };
            let val = (2.0 * PI * 650.0 * t).sin() * d1 + (2.0 * PI * 1050.0 * t).sin() * d2;
            chord_samples.push(val * 0.5);
        }
        let sound_chord_raw = make_wav(&chord_samples, sample_rate);

        // 5. Lose / Explosion (650ms ruido y subgrave)
        let lose_dur = 0.65;
        let n = (sample_rate as f32 * lose_dur) as usize;
        let mut lose_samples = Vec::with_capacity(n);
        let mut seed: u32 = 123456789;
        for i in 0..n {
            let t = i as f32 / sample_rate as f32;
            let decay = (-5.5 * t).exp();
            // LCG simple para ruido blanco consistente
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let noise = ((seed as f32 / u32::MAX as f32) * 2.0 - 1.0) * 0.65;
            let rumble1 = (2.0 * PI * (80.0 - 40.0 * t) * t).sin() * 0.5;
            let rumble2 = (2.0 * PI * 45.0 * t).sin() * 0.35;
            lose_samples.push((noise + rumble1 + rumble2) * decay * 0.8);
        }
        let sound_lose_raw = make_wav(&lose_samples, sample_rate);

        // 6. Win (Fanfarria musical C5, E5, G5, C6)
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

        let sound_click = load_sound_from_bytes(&sound_click_raw).await.expect("Failed to load click sound");
        let sound_flag = load_sound_from_bytes(&sound_flag_raw).await.expect("Failed to load flag sound");
        let sound_unflag = load_sound_from_bytes(&sound_unflag_raw).await.expect("Failed to load unflag sound");
        let sound_chord = load_sound_from_bytes(&sound_chord_raw).await.expect("Failed to load chord sound");
        let sound_lose = load_sound_from_bytes(&sound_lose_raw).await.expect("Failed to load lose sound");
        let sound_win = load_sound_from_bytes(&sound_win_raw).await.expect("Failed to load win sound");

        Self {
            enabled: true,
            sound_click,
            sound_flag,
            sound_unflag,
            sound_chord,
            sound_lose,
            sound_win,
        }
    }

    fn play_sound(&self, sound: &Sound) {
        if self.enabled {
            play_sound_once(sound);
        }
    }

    pub fn play_click(&self) { self.play_sound(&self.sound_click); }
    pub fn play_flag(&self) { self.play_sound(&self.sound_flag); }
    pub fn play_unflag(&self) { self.play_sound(&self.sound_unflag); }
    pub fn play_chord(&self) { self.play_sound(&self.sound_chord); }
    pub fn play_lose(&self) { self.play_sound(&self.sound_lose); }
    pub fn play_win(&self) { self.play_sound(&self.sound_win); }

    pub fn toggle(&mut self) -> bool {
        self.enabled = !self.enabled;
        self.enabled
    }
}

fn make_wav(samples: &[f32], sample_rate: u32) -> Vec<u8> {
    let num_samples = samples.len();
    let subchunk2_size = (num_samples * 2) as u32;
    let chunk_size = 36 + subchunk2_size;
    let byte_rate = sample_rate * 2;

    let mut buf = Vec::with_capacity((44 + subchunk2_size) as usize);

    // RIFF header
    buf.extend_from_slice(b"RIFF");
    buf.extend_from_slice(&chunk_size.to_le_bytes());
    buf.extend_from_slice(b"WAVE");

    // "fmt " sub-chunk
    buf.extend_from_slice(b"fmt ");
    buf.extend_from_slice(&16u32.to_le_bytes()); // Subchunk1Size (16 for PCM)
    buf.extend_from_slice(&1u16.to_le_bytes());  // AudioFormat (1 for PCM)
    buf.extend_from_slice(&1u16.to_le_bytes());  // NumChannels (1 = Mono)
    buf.extend_from_slice(&sample_rate.to_le_bytes());
    buf.extend_from_slice(&byte_rate.to_le_bytes());
    buf.extend_from_slice(&2u16.to_le_bytes());  // BlockAlign (1 * 16/8)
    buf.extend_from_slice(&16u16.to_le_bytes()); // BitsPerSample

    // "data" sub-chunk
    buf.extend_from_slice(b"data");
    buf.extend_from_slice(&subchunk2_size.to_le_bytes());

    for &s in samples {
        let clamped = s.clamp(-1.0, 1.0);
        let val = (clamped * 32767.0) as i16;
        buf.extend_from_slice(&val.to_le_bytes());
    }

    buf
}

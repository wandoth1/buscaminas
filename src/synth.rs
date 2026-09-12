//! Primitivas de síntesis compartidas: codificación WAV, osciladores,
//! envolventes y un filtro paso bajo de un polo.
//!
//! Este módulo no depende de macroquad a propósito: así la generación de audio
//! se puede probar y volcar a disco sin abrir una ventana.

use std::f32::consts::PI;

/// Frecuencia de muestreo de todo el audio del juego.
pub const SAMPLE_RATE: u32 = 22_050;

/// Una nota a renderizar: cuándo empieza, cuánto dura, a qué altura y con qué
/// amplitud. Agrupar los parámetros evita firmas de función interminables.
#[derive(Clone, Copy, Debug)]
pub struct Note {
    /// Inicio en segundos desde el arranque del buffer.
    pub start: f32,
    /// Duración en segundos, envolvente incluida.
    pub dur: f32,
    /// Frecuencia fundamental en Hz.
    pub freq: f32,
    /// Amplitud de pico antes de la normalización final.
    pub amp: f32,
}

/// Generador pseudoaleatorio xorshift32.
///
/// Determinista y propio a propósito: la misma semilla da siempre el mismo
/// tema, así que las compilaciones son reproducibles y los tests estables.
pub struct Rng(u32);

impl Rng {
    pub fn new(seed: u32) -> Self {
        // Un estado de cero se quedaría atascado en cero.
        Self(if seed == 0 { 0x9E37_79B9 } else { seed })
    }

    pub fn next_u32(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        self.0
    }

    /// Flotante en [0, 1).
    pub fn next_f32(&mut self) -> f32 {
        (self.next_u32() >> 8) as f32 / (1u32 << 24) as f32
    }

    /// Flotante en [lo, hi).
    pub fn range(&mut self, lo: f32, hi: f32) -> f32 {
        lo + (hi - lo) * self.next_f32()
    }
}

/// Convierte una nota MIDI a Hz (69 = A4 = 440 Hz).
pub fn midi_to_freq(note: f32) -> f32 {
    440.0 * ((note - 69.0) / 12.0).exp2()
}

/// Rampa de ataque lineal: evita el chasquido de empezar una onda a amplitud
/// plena. Devuelve 1.0 pasado `attack`.
fn attack_ramp(t: f32, attack: f32) -> f32 {
    if attack <= 0.0 || t >= attack {
        1.0
    } else {
        t / attack
    }
}

/// Ventana de la nota en muestras, recortada al buffer.
fn span(buf_len: usize, sr: u32, note: &Note) -> Option<(usize, usize)> {
    let sr_f = sr as f32;
    let from = (note.start * sr_f).round().max(0.0) as usize;
    let to = ((note.start + note.dur) * sr_f).round().max(0.0) as usize;
    let to = to.min(buf_len);
    if from >= to { None } else { Some((from, to)) }
}

/// Campana / piano eléctrico: parciales armónicos con decaimiento exponencial,
/// los agudos apagándose antes que el fundamental. Es el timbre principal del
/// tema relajado.
pub fn render_bell(buf: &mut [f32], sr: u32, note: &Note, decay: f32) {
    let Some((from, to)) = span(buf.len(), sr, note) else {
        return;
    };
    let sr_f = sr as f32;
    // (múltiplo del fundamental, peso, factor de decaimiento extra)
    const PARTIALS: [(f32, f32, f32); 4] = [
        (1.0, 1.0, 1.0),
        (2.0, 0.34, 1.9),
        (3.01, 0.13, 2.8),
        (4.02, 0.05, 3.6),
    ];

    for (i, sample) in buf[from..to].iter_mut().enumerate() {
        let t = i as f32 / sr_f;
        let env = attack_ramp(t, 0.006) * (-decay * t).exp();
        let mut v = 0.0;
        for (mult, weight, dk) in PARTIALS {
            let w = 2.0 * PI * note.freq * mult;
            v += weight * (-decay * dk * t).exp() * (w * t).sin();
        }
        // Segunda voz ligeramente desafinada: da cuerpo, como un coro suave.
        v += 0.45 * (-decay * t).exp() * (2.0 * PI * note.freq * 1.0015 * t).sin();
        *sample += v * env * note.amp;
    }
}

/// Pulsación suave y oscura: triangular, ataque corto y cola media. Es el
/// pulso del tema de concentración.
pub fn render_pluck(buf: &mut [f32], sr: u32, note: &Note, decay: f32) {
    let Some((from, to)) = span(buf.len(), sr, note) else {
        return;
    };
    let sr_f = sr as f32;

    for (i, sample) in buf[from..to].iter_mut().enumerate() {
        let t = i as f32 / sr_f;
        let env = attack_ramp(t, 0.02) * (-decay * t).exp();
        let w = 2.0 * PI * note.freq;
        // Aproximación de triangular con parciales impares.
        let v = (w * t).sin() + (3.0 * w * t).sin() / 9.0 + (5.0 * w * t).sin() / 25.0;
        *sample += v * env * note.amp;
    }
}

/// Colchón armónico: tres osciladores desafinados con ataque lento y caída
/// larga. Sostiene la armonía por debajo de todo lo demás.
pub fn render_pad(buf: &mut [f32], sr: u32, note: &Note, attack: f32) {
    let Some((from, to)) = span(buf.len(), sr, note) else {
        return;
    };
    let sr_f = sr as f32;
    const DETUNE: [f32; 3] = [0.9967, 1.0, 1.0035];

    for (i, sample) in buf[from..to].iter_mut().enumerate() {
        let t = i as f32 / sr_f;
        // Ataque lento y liberación en coseno: llega a cero al final de la nota,
        // que es lo que permite que el bucle empalme.
        let rel = (t / note.dur).clamp(0.0, 1.0);
        let env = attack_ramp(t, attack) * (0.5 + 0.5 * (PI * rel).cos()).powf(1.4);
        let mut v = 0.0;
        for d in DETUNE {
            let w = 2.0 * PI * note.freq * d;
            v += (w * t).sin() + (3.0 * w * t).sin() / 7.0;
        }
        *sample += v * env * note.amp;
    }
}

/// Bajo: seno con un poco de segundo armónico y ataque muy blando.
pub fn render_sub(buf: &mut [f32], sr: u32, note: &Note) {
    let Some((from, to)) = span(buf.len(), sr, note) else {
        return;
    };
    let sr_f = sr as f32;

    for (i, sample) in buf[from..to].iter_mut().enumerate() {
        let t = i as f32 / sr_f;
        let env = attack_ramp(t, 0.05) * (-1.1 * t).exp();
        let w = 2.0 * PI * note.freq;
        let v = (w * t).sin() + 0.18 * (2.0 * w * t).sin();
        *sample += v * env * note.amp;
    }
}

/// Filtro paso bajo de un polo, in situ. `cutoff` en Hz.
pub fn low_pass(buf: &mut [f32], sr: u32, cutoff: f32) {
    let a = 1.0 - (-2.0 * PI * cutoff / sr as f32).exp();
    let mut y = 0.0;
    for sample in buf.iter_mut() {
        y += a * (*sample - y);
        *sample = y;
    }
}

/// Dobla la cola del buffer sobre su principio y lo recorta a `loop_len`.
///
/// Es lo que hace que el bucle no tenga costura: las notas que se salen del
/// final reaparecen al principio, igual que harían si el tema siguiera sonando.
pub fn fold_tail(buf: &mut Vec<f32>, loop_len: usize) {
    if buf.len() <= loop_len {
        buf.resize(loop_len, 0.0);
        return;
    }
    let tail: Vec<f32> = buf[loop_len..].to_vec();
    for (i, v) in tail.iter().enumerate() {
        buf[i % loop_len] += v;
    }
    buf.truncate(loop_len);
}

/// Escala el buffer para que su pico quede en `target`. Sin pico (silencio) no
/// toca nada.
pub fn normalize(buf: &mut [f32], target: f32) {
    let peak = buf.iter().fold(0.0f32, |m, s| m.max(s.abs()));
    if peak <= f32::EPSILON {
        return;
    }
    let gain = target / peak;
    for sample in buf.iter_mut() {
        *sample *= gain;
    }
}

/// Empaqueta muestras en [-1, 1] como un WAV PCM mono de 16 bits.
pub fn make_wav(samples: &[f32], sample_rate: u32) -> Vec<u8> {
    let num_samples = samples.len();
    let subchunk2_size = (num_samples * 2) as u32;
    let chunk_size = 36 + subchunk2_size;
    let byte_rate = sample_rate * 2;

    let mut buf = Vec::with_capacity(44 + subchunk2_size as usize);
    buf.extend_from_slice(b"RIFF");
    buf.extend_from_slice(&chunk_size.to_le_bytes());
    buf.extend_from_slice(b"WAVE");
    buf.extend_from_slice(b"fmt ");
    buf.extend_from_slice(&16u32.to_le_bytes());
    buf.extend_from_slice(&1u16.to_le_bytes());
    buf.extend_from_slice(&1u16.to_le_bytes());
    buf.extend_from_slice(&sample_rate.to_le_bytes());
    buf.extend_from_slice(&byte_rate.to_le_bytes());
    buf.extend_from_slice(&2u16.to_le_bytes());
    buf.extend_from_slice(&16u16.to_le_bytes());
    buf.extend_from_slice(b"data");
    buf.extend_from_slice(&subchunk2_size.to_le_bytes());

    for &s in samples {
        let clamped = s.clamp(-1.0, 1.0);
        let val = (clamped * 32767.0) as i16;
        buf.extend_from_slice(&val.to_le_bytes());
    }

    buf
}

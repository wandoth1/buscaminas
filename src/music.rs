//! Los dos temas de fondo, compuestos y sintetizados en código.
//!
//! Son composiciones originales escritas para este juego: no hay samples ni
//! transcripciones de nada. Cada tema se genera como un bucle cerrado, y la
//! costura se resuelve doblando la cola sobre el principio (`synth::fold_tail`),
//! así que puede repetirse indefinidamente sin salto audible.
//!
//! - [`MusicTrack::Relax`]: fa mayor, 60 BPM, 8 acordes de 2 compases (64 s).
//!   Campanas dispersas sobre un colchón suave, mucho aire entre notas.
//! - [`MusicTrack::Focus`]: la menor, 72 BPM, 6 acordes de 4 compases (80 s).
//!   Colchón ancho, pulso de negras filtrado y grave, sin melodía que reclame
//!   atención.

use crate::synth::{
    self, Note, Rng, SAMPLE_RATE, low_pass, make_wav, midi_to_freq, normalize, render_bell,
    render_pad, render_pluck, render_sub,
};

/// Cola extra que se renderiza más allá del bucle para luego doblarla sobre el
/// principio. Tiene que cubrir la nota más larga de cualquiera de los temas.
const TAIL_SECONDS: f32 = 8.0;

/// Pico al que se normaliza cada tema. Deja margen para que la música no tape
/// los efectos de sonido del juego.
const PEAK: f32 = 0.72;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MusicTrack {
    /// Ligera y relajada, para tener puesta de fondo sin pensar en ella.
    Relax,
    /// Chillout de concentración: plana, sin sobresaltos, para no distraer.
    Focus,
}

impl MusicTrack {
    /// Duración exacta del bucle en segundos.
    pub fn loop_seconds(self) -> f32 {
        match self {
            // 8 acordes x 2 compases x 4 tiempos a 60 BPM.
            MusicTrack::Relax => 64.0,
            // 6 acordes x 4 compases x 4 tiempos a 72 BPM.
            MusicTrack::Focus => 6.0 * 4.0 * 4.0 * 60.0 / 72.0,
        }
    }

    /// Renderiza el bucle como muestras en [-1, 1].
    pub fn render(self) -> Vec<f32> {
        match self {
            MusicTrack::Relax => render_relax(),
            MusicTrack::Focus => render_focus(),
        }
    }

    /// Renderiza el bucle ya empaquetado como WAV PCM mono de 16 bits.
    pub fn wav(self) -> Vec<u8> {
        make_wav(&self.render(), SAMPLE_RATE)
    }
}

/// Longitud del bucle en muestras.
fn loop_len(track: MusicTrack) -> usize {
    (track.loop_seconds() * SAMPLE_RATE as f32).round() as usize
}

/// Buffer de trabajo: el bucle más la cola que se doblará encima.
fn work_buffer(track: MusicTrack) -> Vec<f32> {
    let total = track.loop_seconds() + TAIL_SECONDS;
    vec![0.0; (total * SAMPLE_RATE as f32).round() as usize]
}

// =========================================================================
// Tema 1: Relax — fa mayor, 60 BPM
// =========================================================================

/// Un acorde del tema relajado, con el registro de cada capa ya elegido.
struct RelaxChord {
    /// Fundamental del bajo (nota MIDI).
    bass: f32,
    /// Notas del colchón.
    pad: [f32; 4],
    /// Reserva de notas para las campanas, de grave a agudo.
    arp: [f32; 5],
    /// Notas de melodía; vacío deja el acorde respirando sin melodía.
    melody: &'static [f32],
}

/// Fmaj9 – Dm7 – Bbmaj7 – C6/9 – Am7 – Bbmaj7 – Gm7 – Csus4.
///
/// Diatónico en fa, sin dominantes tensos: la vuelta al principio del bucle no
/// suena a "otra vez", solo continúa.
const RELAX_CHORDS: [RelaxChord; 8] = [
    RelaxChord {
        bass: 41.0,
        pad: [60.0, 65.0, 69.0, 72.0],
        arp: [72.0, 76.0, 79.0, 81.0, 84.0],
        melody: &[],
    },
    RelaxChord {
        bass: 38.0,
        pad: [57.0, 62.0, 65.0, 69.0],
        arp: [72.0, 74.0, 77.0, 81.0, 84.0],
        melody: &[],
    },
    RelaxChord {
        bass: 46.0,
        pad: [58.0, 62.0, 65.0, 69.0],
        arp: [70.0, 74.0, 77.0, 81.0, 86.0],
        melody: &[81.0, 77.0, 74.0],
    },
    RelaxChord {
        bass: 36.0,
        pad: [55.0, 60.0, 64.0, 69.0],
        arp: [72.0, 74.0, 76.0, 79.0, 81.0],
        melody: &[79.0, 76.0, 72.0],
    },
    RelaxChord {
        bass: 45.0,
        pad: [57.0, 60.0, 64.0, 67.0],
        arp: [72.0, 76.0, 79.0, 81.0, 84.0],
        melody: &[],
    },
    RelaxChord {
        bass: 46.0,
        pad: [58.0, 62.0, 65.0, 69.0],
        arp: [74.0, 77.0, 81.0, 86.0, 89.0],
        melody: &[],
    },
    RelaxChord {
        bass: 43.0,
        pad: [55.0, 58.0, 62.0, 65.0],
        arp: [70.0, 74.0, 77.0, 79.0, 82.0],
        melody: &[77.0, 74.0, 70.0],
    },
    RelaxChord {
        bass: 36.0,
        pad: [55.0, 60.0, 65.0, 67.0],
        arp: [72.0, 77.0, 79.0, 84.0, 88.0],
        melody: &[72.0, 76.0, 77.0],
    },
];

fn render_relax() -> Vec<f32> {
    let track = MusicTrack::Relax;
    let mut buf = work_buffer(track);
    let sr = SAMPLE_RATE;
    let mut rng = Rng::new(0x5EED_1A3F);

    // 2 compases de 4/4 a 60 BPM.
    const CHORD_SECONDS: f32 = 8.0;
    // Esqueleto rítmico de las campanas dentro del acorde, en segundos.
    const ARP_OFFSETS: [f32; 6] = [0.0, 1.5, 2.5, 4.0, 5.5, 6.5];
    // Contorno: qué nota de la reserva toca en cada golpe. Sube y vuelve.
    const ARP_CONTOUR: [usize; 6] = [0, 2, 1, 3, 2, 4];
    const MELODY_OFFSETS: [f32; 3] = [0.5, 3.0, 5.5];

    for (i, chord) in RELAX_CHORDS.iter().enumerate() {
        let base = i as f32 * CHORD_SECONDS;

        // Bajo: una nota por acorde, blanda y larga.
        render_sub(
            &mut buf,
            sr,
            &Note {
                start: base,
                dur: 4.5,
                freq: midi_to_freq(chord.bass),
                amp: 0.22,
            },
        );

        // Colchón: se solapa con el acorde siguiente para que no haya huecos.
        for note in chord.pad {
            render_pad(
                &mut buf,
                sr,
                &Note {
                    start: base,
                    dur: CHORD_SECONDS + 1.2,
                    freq: midi_to_freq(note),
                    amp: 0.055,
                },
                1.6,
            );
        }

        // Campanas: el esqueleto se humaniza un poco para que no suene a reja.
        for (step, offset) in ARP_OFFSETS.iter().enumerate() {
            let jitter = rng.range(-0.05, 0.05);
            let pitch = chord.arp[ARP_CONTOUR[step]];
            render_bell(
                &mut buf,
                sr,
                &Note {
                    start: base + offset + jitter,
                    dur: 3.2,
                    freq: midi_to_freq(pitch),
                    amp: rng.range(0.13, 0.18),
                },
                1.15,
            );
        }

        // Melodía: solo en los acordes que la traen, notas largas y pocas.
        for (step, pitch) in chord.melody.iter().enumerate() {
            render_bell(
                &mut buf,
                sr,
                &Note {
                    start: base + MELODY_OFFSETS[step.min(MELODY_OFFSETS.len() - 1)],
                    dur: 3.6,
                    freq: midi_to_freq(*pitch),
                    amp: 0.13,
                },
                0.85,
            );
        }
    }

    // Se filtra antes de doblar la cola, para que cabeza y cola queden con el
    // mismo timbre y el empalme siga siendo continuo.
    low_pass(&mut buf, sr, 6000.0);
    synth::fold_tail(&mut buf, loop_len(track));
    normalize(&mut buf, PEAK);
    buf
}

// =========================================================================
// Tema 2: Focus — la menor, 72 BPM
// =========================================================================

/// Un acorde del tema de concentración.
struct FocusChord {
    bass: f32,
    pad: [f32; 4],
    /// Patrón de negras, una entrada por tiempo del compás.
    arp: [f32; 4],
    /// Campanas agudas ocasionales; vacío deja el acorde liso.
    bell: &'static [f32],
}

/// Am9 – Fmaj7 – C2 – Em7 – Fmaj7 – G6/9.
///
/// Bucle modal sin resolución dominante: no "pide" nada, que es justo lo que
/// se busca en música de fondo para concentrarse.
const FOCUS_CHORDS: [FocusChord; 6] = [
    FocusChord {
        bass: 45.0,
        pad: [57.0, 60.0, 64.0, 71.0],
        arp: [69.0, 72.0, 76.0, 72.0],
        bell: &[],
    },
    FocusChord {
        bass: 41.0,
        pad: [57.0, 60.0, 65.0, 69.0],
        arp: [65.0, 69.0, 72.0, 69.0],
        bell: &[84.0, 88.0],
    },
    FocusChord {
        bass: 36.0,
        pad: [55.0, 59.0, 64.0, 67.0],
        arp: [67.0, 72.0, 74.0, 71.0],
        bell: &[],
    },
    FocusChord {
        bass: 40.0,
        pad: [55.0, 59.0, 62.0, 67.0],
        arp: [67.0, 71.0, 74.0, 71.0],
        bell: &[],
    },
    FocusChord {
        bass: 41.0,
        pad: [57.0, 60.0, 65.0, 69.0],
        arp: [65.0, 69.0, 72.0, 77.0],
        bell: &[89.0, 84.0],
    },
    FocusChord {
        bass: 43.0,
        pad: [55.0, 59.0, 62.0, 69.0],
        arp: [67.0, 71.0, 74.0, 69.0],
        bell: &[],
    },
];

fn render_focus() -> Vec<f32> {
    let track = MusicTrack::Focus;
    let sr = SAMPLE_RATE;
    // Dos capas: la oscura va filtrada a fondo, las campanas se quedan limpias.
    let mut dark = work_buffer(track);
    let mut bright = work_buffer(track);
    let mut rng = Rng::new(0xC0FF_EE17);

    const BEAT: f32 = 60.0 / 72.0;
    const BAR: f32 = 4.0 * BEAT;
    const BARS_PER_CHORD: usize = 4;
    let chord_seconds = BARS_PER_CHORD as f32 * BAR;
    const BELL_OFFSETS: [f32; 2] = [5.0, 9.5];

    for (i, chord) in FOCUS_CHORDS.iter().enumerate() {
        let base = i as f32 * chord_seconds;

        // Colchón ancho, ataque muy lento, solapando con el acorde siguiente.
        for note in chord.pad {
            render_pad(
                &mut dark,
                sr,
                &Note {
                    start: base,
                    dur: chord_seconds + 1.6,
                    freq: midi_to_freq(note),
                    amp: 0.05,
                },
                2.4,
            );
        }

        for bar in 0..BARS_PER_CHORD {
            let bar_start = base + bar as f32 * BAR;

            // Un latido de bajo por compás.
            render_sub(
                &mut dark,
                sr,
                &Note {
                    start: bar_start,
                    dur: 3.0,
                    freq: midi_to_freq(chord.bass),
                    amp: 0.18,
                },
            );

            // Pulso de negras: acento suave en el primer tiempo.
            for beat in 0..4 {
                let amp = if beat == 0 { 0.075 } else { 0.055 };
                render_pluck(
                    &mut dark,
                    sr,
                    &Note {
                        start: bar_start + beat as f32 * BEAT,
                        dur: 0.95,
                        freq: midi_to_freq(chord.arp[beat]),
                        amp: amp * rng.range(0.9, 1.1),
                    },
                    2.3,
                );
            }
        }

        // Campanas: brillo puntual, muy por debajo del resto.
        for (step, pitch) in chord.bell.iter().enumerate() {
            render_bell(
                &mut bright,
                sr,
                &Note {
                    start: base + BELL_OFFSETS[step.min(BELL_OFFSETS.len() - 1)],
                    dur: 4.5,
                    freq: midi_to_freq(*pitch),
                    amp: 0.045,
                },
                0.75,
            );
        }
    }

    low_pass(&mut dark, sr, 1500.0);
    low_pass(&mut bright, sr, 7000.0);

    let mut buf = dark;
    for (sample, b) in buf.iter_mut().zip(bright.iter()) {
        *sample += b;
    }

    synth::fold_tail(&mut buf, loop_len(track));

    // Respiración lenta: dos ciclos exactos por bucle, así que el LFO vale lo
    // mismo al principio y al final y no rompe el empalme.
    let len = buf.len() as f32;
    for (i, sample) in buf.iter_mut().enumerate() {
        let phase = 2.0 * std::f32::consts::PI * 2.0 * (i as f32 / len);
        *sample *= 0.9 + 0.1 * phase.sin();
    }

    normalize(&mut buf, PEAK);
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    const TRACKS: [MusicTrack; 2] = [MusicTrack::Relax, MusicTrack::Focus];

    #[test]
    fn loop_length_matches_the_declared_tempo() {
        for track in TRACKS {
            let samples = track.render();
            let expected = (track.loop_seconds() * SAMPLE_RATE as f32).round() as usize;
            assert_eq!(samples.len(), expected, "{track:?}");
        }
    }

    #[test]
    fn nothing_clips() {
        for track in TRACKS {
            let peak = track.render().iter().fold(0.0f32, |m, s| m.max(s.abs()));
            assert!(peak <= 1.0, "{track:?} satura con pico {peak}");
            // Si el pico se queda muy por debajo del objetivo, la normalización
            // no ha hecho su trabajo y el tema sonaría inaudible.
            assert!(peak > PEAK - 0.01, "{track:?} sale demasiado bajo: {peak}");
        }
    }

    #[test]
    fn the_loop_seam_has_no_jump() {
        // El salto entre la última muestra y la primera no debe ser mayor que
        // el mayor salto que ya ocurre dentro del tema: si lo fuera, se oiría
        // un chasquido en cada repetición.
        for track in TRACKS {
            let samples = track.render();
            let seam = (samples[0] - samples[samples.len() - 1]).abs();
            let worst_inside = samples
                .windows(2)
                .fold(0.0f32, |m, w| m.max((w[1] - w[0]).abs()));
            assert!(
                seam <= worst_inside,
                "{track:?}: costura {seam} peor que el salto interno máximo {worst_inside}"
            );
        }
    }

    #[test]
    fn rendering_is_deterministic() {
        for track in TRACKS {
            assert_eq!(track.render(), track.render(), "{track:?}");
        }
    }

    #[test]
    fn wav_header_declares_the_rendered_length() {
        for track in TRACKS {
            let wav = track.wav();
            let samples = track.render().len();
            assert_eq!(&wav[0..4], b"RIFF");
            assert_eq!(&wav[8..12], b"WAVE");
            let data_size = u32::from_le_bytes([wav[40], wav[41], wav[42], wav[43]]) as usize;
            assert_eq!(data_size, samples * 2, "{track:?}");
            assert_eq!(wav.len(), 44 + samples * 2, "{track:?}");
        }
    }
}

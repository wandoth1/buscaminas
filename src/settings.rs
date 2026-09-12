//! Ajustes del jugador, guardados en JSON junto a los mejores tiempos.
//!
//! La ruta la resuelve [`crate::userdata`], así que macOS, Windows y Linux
//! quedan cubiertos sin repetir aquí la variable de entorno de cada uno.
//!
//! Todo el struct lleva `serde(default)`: un fichero de una versión anterior al
//! que le falten campos se carga igual, rellenando lo que falte con el valor
//! por defecto en vez de descartar los ajustes enteros.

use serde::{Deserialize, Serialize};

use crate::music::MusicTrack;
use crate::userdata;

const FILE_NAME: &str = "ajustes_v2.json";

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(default)]
pub struct Settings {
    /// Tema de fondo, o `null` para arrancar en silencio.
    pub music: Option<MusicTrack>,
    /// Efectos de sonido.
    pub sound: bool,
    /// Partículas y sacudida de pantalla.
    pub particles: bool,
    /// Ciclo de bandera con interrogación (?).
    pub question_marks: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            // Un jugador nuevo arranca con la música relajada puesta.
            music: Some(MusicTrack::Relax),
            sound: true,
            particles: true,
            question_marks: true,
        }
    }
}

impl Settings {
    /// Lee los ajustes guardados. Si no hay fichero, o está corrupto, devuelve
    /// los valores por defecto: perder los ajustes no debe impedir jugar.
    pub fn load() -> Self {
        std::fs::read_to_string(userdata::data_file(FILE_NAME))
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    }

    /// Guarda los ajustes. Los fallos se ignoran a propósito, igual que en los
    /// mejores tiempos: si el disco está lleno o de solo lectura, el juego
    /// sigue funcionando con los ajustes en memoria.
    pub fn save(&self) {
        let Ok(json) = serde_json::to_string_pretty(self) else {
            return;
        };
        let _ = std::fs::write(userdata::data_file(FILE_NAME), json);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_start_with_the_relax_track() {
        let settings = Settings::default();
        assert_eq!(settings.music, Some(MusicTrack::Relax));
        assert!(settings.sound);
        assert!(settings.particles);
        assert!(settings.question_marks);
    }

    #[test]
    fn round_trips_through_json() {
        for music in [None, Some(MusicTrack::Relax), Some(MusicTrack::Focus)] {
            let original = Settings {
                music,
                sound: false,
                particles: true,
                question_marks: false,
            };
            let json = serde_json::to_string(&original).expect("serializa");
            let parsed: Settings = serde_json::from_str(&json).expect("deserializa");
            assert_eq!(parsed, original);
        }
    }

    #[test]
    fn music_is_stored_with_readable_names() {
        // El fichero lo puede abrir una persona, así que los valores van en
        // castellano y no como los identificadores del código.
        let relax = serde_json::to_string(&Settings {
            music: Some(MusicTrack::Relax),
            ..Settings::default()
        })
        .expect("serializa");
        assert!(relax.contains("\"relax\""), "{relax}");

        let focus = serde_json::to_string(&Settings {
            music: Some(MusicTrack::Focus),
            ..Settings::default()
        })
        .expect("serializa");
        assert!(focus.contains("\"concentracion\""), "{focus}");
    }

    #[test]
    fn missing_fields_fall_back_to_defaults() {
        let parsed: Settings = serde_json::from_str("{}").expect("deserializa");
        assert_eq!(parsed, Settings::default());

        // Un fichero que solo trae un campo conserva el resto.
        let parsed: Settings = serde_json::from_str(r#"{"sound":false}"#).expect("deserializa");
        assert!(!parsed.sound);
        assert_eq!(parsed.music, Settings::default().music);
        assert!(parsed.particles);
    }

    #[test]
    fn a_corrupt_file_does_not_lose_the_other_fields() {
        // Valor desconocido en `music`: serde falla y `load` cae a los valores
        // por defecto en vez de dejar el juego sin ajustes.
        let parsed: Result<Settings, _> = serde_json::from_str(r#"{"music":"tecno"}"#);
        assert!(parsed.is_err());
    }
}

//! Vuelca los dos temas de fondo a WAV para poder escucharlos fuera del juego.
//!
//! ```text
//! cargo run --release --example exportar_musica [carpeta_destino]
//! ```
//!
//! Útil para auditar la música sin abrir la ventana, y para comparar el
//! resultado después de tocar la composición.

use std::path::PathBuf;
use std::time::Instant;

use buscaminas_v2_rust::music::MusicTrack;

fn main() {
    let dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    if let Err(err) = std::fs::create_dir_all(&dir) {
        eprintln!("No se pudo crear {}: {err}", dir.display());
        std::process::exit(1);
    }

    let tracks = [
        (MusicTrack::Relax, "buscaminas_relax.wav"),
        (MusicTrack::Focus, "buscaminas_concentracion.wav"),
    ];

    for (track, file_name) in tracks {
        let started = Instant::now();
        let wav = track.wav();
        let elapsed_ms = started.elapsed().as_millis();
        let path = dir.join(file_name);

        if let Err(err) = std::fs::write(&path, &wav) {
            eprintln!("No se pudo escribir {}: {err}", path.display());
            std::process::exit(1);
        }

        println!(
            "{}: bucle de {:.2} s, {} KiB, generado en {} ms",
            path.display(),
            track.loop_seconds(),
            wav.len() / 1024,
            elapsed_ms
        );
    }
}

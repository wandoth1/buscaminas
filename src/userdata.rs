//! Dónde guardar los ficheros del jugador, con una sola definición por
//! plataforma.
//!
//! Tanto los mejores tiempos como los ajustes viven aquí, así que la variable
//! de entorno de cada sistema está escrita una vez y no en cada módulo:
//!
//! - **macOS**: `$HOME/Library/Application Support/Buscaminas`
//! - **Windows**: `%LOCALAPPDATA%\Buscaminas`
//! - **Resto**: `$XDG_DATA_HOME/buscaminas`, o `$HOME/.local/share/buscaminas`
//!
//! Si la carpeta preferida no se puede crear (portátil en un USB, permisos
//! raros, variable sin definir), se cae al directorio del ejecutable.

use std::fs;
use std::path::{Path, PathBuf};

/// Ruta completa de un fichero de datos del jugador.
pub fn data_file(file_name: &str) -> PathBuf {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(Path::to_path_buf))
        .unwrap_or_else(|| PathBuf::from("."));

    resolve(preferred_dir(), &exe_dir, file_name)
}

/// Carpeta de datos del sistema, sin crear todavía.
fn preferred_dir() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        std::env::var_os("HOME").map(|home| {
            PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join("Buscaminas")
        })
    }

    #[cfg(target_os = "windows")]
    {
        std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .map(|path| path.join("Buscaminas"))
    }

    // El resto sigue la convención XDG, con el nombre en minúsculas.
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME")
                    .map(PathBuf::from)
                    .map(|path| path.join(".local").join("share"))
            })
            .map(|path| path.join("buscaminas"))
    }
}

/// Usa la carpeta preferida si se puede crear; si no, el directorio del
/// ejecutable.
fn resolve(preferred: Option<PathBuf>, exe_dir: &Path, file_name: &str) -> PathBuf {
    if let Some(dir) = preferred
        && fs::create_dir_all(&dir).is_ok()
    {
        return dir.join(file_name);
    }
    exe_dir.join(file_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn falls_back_to_the_executable_directory() {
        let exe_dir = Path::new("/tmp/buscaminas-exe");
        let path = resolve(None, exe_dir, "ajustes.json");
        assert_eq!(path, exe_dir.join("ajustes.json"));
    }

    #[test]
    fn uses_the_preferred_directory_when_it_can_be_created() {
        let dir = std::env::temp_dir().join("buscaminas-userdata-test");
        let path = resolve(Some(dir.clone()), Path::new("."), "ajustes.json");
        assert_eq!(path, dir.join("ajustes.json"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn data_file_keeps_the_requested_name() {
        let path = data_file("mejores_tiempos_v2.json");
        assert_eq!(
            path.file_name().and_then(|n| n.to_str()),
            Some("mejores_tiempos_v2.json")
        );
    }
}

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScoreEntry {
    pub name: String,
    pub time: u32,
    pub date: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HighScoreData {
    pub principiante: Vec<ScoreEntry>,
    pub intermedio: Vec<ScoreEntry>,
    pub experto: Vec<ScoreEntry>,
}

impl Default for HighScoreData {
    fn default() -> Self {
        Self {
            principiante: vec![
                ScoreEntry { name: "Experto 9x9".to_string(), time: 15, date: "2026-01-01".to_string() },
                ScoreEntry { name: "Buscador".to_string(), time: 25, date: "2026-01-01".to_string() },
                ScoreEntry { name: "Novato".to_string(), time: 45, date: "2026-01-01".to_string() },
            ],
            intermedio: vec![
                ScoreEntry { name: "Minero Pro".to_string(), time: 65, date: "2026-01-01".to_string() },
                ScoreEntry { name: "Desactivador".to_string(), time: 95, date: "2026-01-01".to_string() },
                ScoreEntry { name: "Iniciado".to_string(), time: 140, date: "2026-01-01".to_string() },
            ],
            experto: vec![
                ScoreEntry { name: "Leyenda".to_string(), time: 160, date: "2026-01-01".to_string() },
                ScoreEntry { name: "Gran Maestro".to_string(), time: 220, date: "2026-01-01".to_string() },
                ScoreEntry { name: "Superviviente".to_string(), time: 320, date: "2026-01-01".to_string() },
            ],
        }
    }
}

pub struct HighScoreManager {
    pub data: HighScoreData,
    file_path: PathBuf,
}

impl HighScoreManager {
    pub fn new() -> Self {
        let exe_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."));

        #[cfg(target_os = "macos")]
        let file_path = if let Ok(home) = std::env::var("HOME") {
            let app_support = PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join("Buscaminas");
            if fs::create_dir_all(&app_support).is_ok() {
                app_support.join("mejores_tiempos_v2.json")
            } else {
                exe_dir.join("mejores_tiempos_v2.json")
            }
        } else {
            exe_dir.join("mejores_tiempos_v2.json")
        };

        #[cfg(not(target_os = "macos"))]
        let file_path = exe_dir.join("mejores_tiempos_v2.json");

        let mut mgr = Self {
            data: HighScoreData::default(),
            file_path,
        };
        mgr.load();
        mgr
    }

    pub fn load(&mut self) {
        if let Ok(content) = fs::read_to_string(&self.file_path) {
            if let Ok(data) = serde_json::from_str::<HighScoreData>(&content) {
                self.data = data;
            }
        }
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.data) {
            if let Some(parent) = self.file_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::write(&self.file_path, json);
        }
    }

    pub fn is_record(&self, diff_id: &str, seconds: u32) -> bool {
        let list = match diff_id {
            "principiante" => &self.data.principiante,
            "intermedio" => &self.data.intermedio,
            "experto" => &self.data.experto,
            _ => return false,
        };
        if list.len() < 5 {
            return true;
        }
        list.iter().any(|entry| seconds < entry.time)
    }

    pub fn add_score(&mut self, diff_id: &str, mut name: String, seconds: u32) {
        name = name.trim().to_string();
        if name.is_empty() {
            name = "Anónimo".to_string();
        }
        if name.chars().count() > 16 {
            name = name.chars().take(16).collect();
        }

        let entry = ScoreEntry {
            name,
            time: seconds,
            date: current_utc_date(),
        };

        let list = match diff_id {
            "principiante" => &mut self.data.principiante,
            "intermedio" => &mut self.data.intermedio,
            "experto" => &mut self.data.experto,
            _ => return,
        };

        list.push(entry);
        list.sort_by_key(|e| e.time);
        list.truncate(5);
        self.save();
    }

    pub fn get_scores(&self, diff_id: &str) -> &[ScoreEntry] {
        match diff_id {
            "principiante" => &self.data.principiante,
            "intermedio" => &self.data.intermedio,
            "experto" => &self.data.experto,
            _ => &[],
        }
    }

    pub fn reset_defaults(&mut self) {
        self.data = HighScoreData::default();
        self.save();
    }
}

fn current_utc_date() -> String {
    let days_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
        / 86_400;
    let (year, month, day) = civil_from_days(days_since_epoch);
    format!("{year:04}-{month:02}-{day:02}")
}

fn civil_from_days(days_since_epoch: i64) -> (i64, u32, u32) {
    let z = days_since_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let day_of_era = z - era * 146_097;
    let year_of_era = (day_of_era - day_of_era / 1_460 + day_of_era / 36_524
        - day_of_era / 146_096)
        / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    if month <= 2 {
        year += 1;
    }
    (year, month as u32, day as u32)
}

#[cfg(test)]
mod tests {
    use super::civil_from_days;

    #[test]
    fn converts_unix_epoch_date() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
    }

    #[test]
    fn converts_known_2026_date() {
        assert_eq!(civil_from_days(20_343), (2025, 9, 12));
    }
}

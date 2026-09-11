use std::path::PathBuf;
use std::fs;
use serde::{Serialize, Deserialize};

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
        if list.len() < 3 {
            return true;
        }
        list.iter().any(|entry| seconds < entry.time)
    }

    pub fn add_score(&mut self, diff_id: &str, mut name: String, seconds: u32) {
        if name.trim().is_empty() {
            name = "Anónimo".to_string();
        }
        if name.chars().count() > 16 {
            name = name.chars().take(16).collect();
        }
        let today = "2026-09-11".to_string();
        let entry = ScoreEntry { name, time: seconds, date: today };

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

from pathlib import Path


def replace_once(path: str, old: str, new: str) -> None:
    p = Path(path)
    text = p.read_text()
    if old not in text:
        raise SystemExit(f"{path}: expected block not found: {old[:120]!r}")
    p.write_text(text.replace(old, new, 1))


# engine.rs
path = Path("src/engine.rs")
text = path.read_text()
cell_impl = """impl Cell {
    pub fn new() -> Self {
        Self {
            is_mine: false,
            adjacent_mines: 0,
            state: CellState::Hidden,
        }
    }
}
"""
cell_with_default = cell_impl + """
impl Default for Cell {
    fn default() -> Self {
        Self::new()
    }
}
"""
if cell_impl not in text:
    raise SystemExit("engine.rs: Cell impl block not found")
text = text.replace(cell_impl, cell_with_default, 1)

nested_resume = """        if let Some(paused_at) = self.paused_at.take() {
            if let Some(start_time) = &mut self.start_time {
                *start_time += (now - paused_at).max(0.0);
            }
        }
"""
collapsed_resume = """        if let Some(paused_at) = self.paused_at.take()
            && let Some(start_time) = &mut self.start_time
        {
            *start_time += (now - paused_at).max(0.0);
        }
"""
if nested_resume not in text:
    raise SystemExit("engine.rs: resume block not found")
text = text.replace(nested_resume, collapsed_resume, 1)
path.write_text(text)


# highscores.rs
replace_once(
    "src/highscores.rs",
    """        if let Ok(content) = fs::read_to_string(&self.file_path) {
            if let Ok(data) = serde_json::from_str::<HighScoreData>(&content) {
                self.data = data;
            }
        }
""",
    """        if let Ok(content) = fs::read_to_string(&self.file_path)
            && let Ok(data) = serde_json::from_str::<HighScoreData>(&content)
        {
            self.data = data;
        }
""",
)
replace_once(
    "src/highscores.rs",
    """    if let Some(dir) = preferred_dir {
        if fs::create_dir_all(&dir).is_ok() {
            return dir.join("mejores_tiempos_v2.json");
        }
    }
""",
    """    if let Some(dir) = preferred_dir
        && fs::create_dir_all(&dir).is_ok()
    {
        return dir.join("mejores_tiempos_v2.json");
    }
""",
)


# sound.rs
replace_once(
    "src/sound.rs",
    """        if self.enabled {
            if let Some(sound) = sound {
                play_sound_once(sound);
            }
        }
""",
    """        if self.enabled
            && let Some(sound) = sound
        {
            play_sound_once(sound);
        }
""",
)


# ui.rs: keep the private renderer explicit and clean harmless legacy lints.
path = Path("src/ui.rs")
text = path.read_text()
needle = "    fn draw_dropdown(\n"
if needle not in text:
    raise SystemExit("ui.rs: draw_dropdown not found")
text = text.replace(needle, "    #[allow(clippy::too_many_arguments)]\n" + needle, 1)
replacements = {
    "draw_text(&cols.to_string(),": "draw_text(cols.to_string(),",
    "draw_text(&rows.to_string(),": "draw_text(rows.to_string(),",
    "draw_text(&mines.to_string(),": "draw_text(mines.to_string(),",
    "draw_text(&(i + 1).to_string(),": "draw_text((i + 1).to_string(),",
    'draw_text(&format!("{}s", s.time),': 'draw_text(format!("{}s", s.time),',
    '&format!("¡Felicidades! Ganaste en {}", diff_name),': 'format!("¡Felicidades! Ganaste en {}", diff_name),',
    '&format!("Tiempo récord: {} segundos", seconds),': 'format!("Tiempo récord: {} segundos", seconds),',
    "if ((get_time() * 2.0) as usize) % 2 == 0 {": "if ((get_time() * 2.0) as usize).is_multiple_of(2) {",
}
for old, new in replacements.items():
    if old not in text:
        raise SystemExit(f"ui.rs: expected Clippy cleanup not found: {old!r}")
    text = text.replace(old, new, 1)
path.write_text(text)


# main.rs
replace_once(
    "src/main.rs",
    """                if let Some((r, c)) = hovered_cell {
                    if !mouse_left_down {
                        match board.toggle_flag(r, c) {
                            FlagAction::Flag => sound.play_flag(),
                            FlagAction::Unflag | FlagAction::Question => sound.play_unflag(),
                            FlagAction::None => {}
                        }
                    }
                }
""",
    """                if let Some((r, c)) = hovered_cell
                    && !mouse_left_down
                {
                    match board.toggle_flag(r, c) {
                        FlagAction::Flag => sound.play_flag(),
                        FlagAction::Unflag | FlagAction::Question => sound.play_unflag(),
                        FlagAction::None => {}
                    }
                }
""",
)

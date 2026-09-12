from pathlib import Path

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

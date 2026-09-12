use crate::constants::{CellState, GameState};
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub is_mine: bool,
    pub adjacent_mines: u8,
    pub state: CellState,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            is_mine: false,
            adjacent_mines: 0,
            state: CellState::Hidden,
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RevealResult {
    Ok(Vec<(usize, usize)>),
    Mine(usize, usize),
    Win(Vec<(usize, usize)>),
    None,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FlagAction {
    Flag,
    Unflag,
    Question,
    None,
}

pub struct Board {
    pub cols: usize,
    pub rows: usize,
    pub total_mines: usize,
    pub grid: Vec<Cell>,
    pub game_state: GameState,
    pub start_time: Option<f64>,
    pub end_time: Option<f64>,
    pub paused_at: Option<f64>,
    pub flags_count: usize,
    pub revealed_count: usize,
    pub exploded_cell: Option<(usize, usize)>,
    pub first_click: bool,
    pub allow_question: bool,
}

impl Board {
    pub fn new(cols: usize, rows: usize, total_mines: usize) -> Self {
        let count = cols * rows;
        let clamped_mines = if count > 1 {
            total_mines.clamp(1, count - 1)
        } else {
            0
        };
        Self {
            cols,
            rows,
            total_mines: clamped_mines,
            grid: vec![Cell::new(); count],
            game_state: GameState::Ready,
            start_time: None,
            end_time: None,
            paused_at: None,
            flags_count: 0,
            revealed_count: 0,
            exploded_cell: None,
            first_click: true,
            allow_question: true,
        }
    }

    pub fn reset(&mut self, cols: usize, rows: usize, total_mines: usize) {
        self.cols = cols;
        self.rows = rows;
        let count = cols * rows;
        self.total_mines = if count > 1 {
            total_mines.clamp(1, count - 1)
        } else {
            0
        };
        self.grid = vec![Cell::new(); count];
        self.game_state = GameState::Ready;
        self.start_time = None;
        self.end_time = None;
        self.paused_at = None;
        self.flags_count = 0;
        self.revealed_count = 0;
        self.exploded_cell = None;
        self.first_click = true;
    }

    #[inline(always)]
    pub fn idx(&self, r: usize, c: usize) -> usize {
        r * self.cols + c
    }

    pub fn get_neighbors(&self, r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::with_capacity(8);
        let r_i = r as isize;
        let c_i = c as isize;
        let rows_i = self.rows as isize;
        let cols_i = self.cols as isize;

        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let nr = r_i + dr;
                let nc = c_i + dc;
                if nr >= 0 && nr < rows_i && nc >= 0 && nc < cols_i {
                    neighbors.push((nr as usize, nc as usize));
                }
            }
        }
        neighbors
    }

    fn generate_mines(&mut self, safe_r: usize, safe_c: usize) {
        let total_cells = self.rows * self.cols;
        let neighbors = self.get_neighbors(safe_r, safe_c);
        let mut forbidden = Vec::with_capacity(neighbors.len() + 1);
        forbidden.push((safe_r, safe_c));
        forbidden.extend(neighbors);

        // Candidatos
        let mut candidates = Vec::with_capacity(total_cells);
        let can_spare_neighbors = total_cells - forbidden.len() >= self.total_mines;

        for r in 0..self.rows {
            for c in 0..self.cols {
                if can_spare_neighbors {
                    if !forbidden.contains(&(r, c)) {
                        candidates.push((r, c));
                    }
                } else if (r, c) != (safe_r, safe_c) {
                    candidates.push((r, c));
                }
            }
        }

        // Barajado Fisher-Yates rápido con semilla
        let n = candidates.len();
        for i in 0..self.total_mines.min(n) {
            let j = i + (macroquad::rand::gen_range(0, (n - i) as i32) as usize);
            candidates.swap(i, j);
            let (mr, mc) = candidates[i];
            let idx = self.idx(mr, mc);
            self.grid[idx].is_mine = true;
        }

        // Calcular minas adyacentes
        for r in 0..self.rows {
            for c in 0..self.cols {
                let idx = self.idx(r, c);
                if !self.grid[idx].is_mine {
                    let count = self
                        .get_neighbors(r, c)
                        .iter()
                        .filter(|&&(nr, nc)| self.grid[self.idx(nr, nc)].is_mine)
                        .count() as u8;
                    self.grid[idx].adjacent_mines = count;
                }
            }
        }
    }

    pub fn reveal(&mut self, r: usize, c: usize, now: f64) -> RevealResult {
        if self.game_state == GameState::Won || self.game_state == GameState::Lost {
            return RevealResult::None;
        }
        if r >= self.rows || c >= self.cols {
            return RevealResult::None;
        }

        let idx = self.idx(r, c);
        if !matches!(
            self.grid[idx].state,
            CellState::Hidden | CellState::Question
        ) {
            return RevealResult::None;
        }

        if self.first_click {
            self.generate_mines(r, c);
            self.first_click = false;
            self.game_state = GameState::Playing;
            self.start_time = Some(now);
            self.paused_at = None;
        }

        if self.grid[idx].is_mine {
            self.grid[idx].state = CellState::Revealed;
            self.game_state = GameState::Lost;
            self.end_time = Some(now);
            self.exploded_cell = Some((r, c));
            return RevealResult::Mine(r, c);
        }

        let mut newly_revealed = Vec::new();
        let mut queue = VecDeque::new();

        self.grid[idx].state = CellState::Revealed;
        self.revealed_count += 1;
        newly_revealed.push((r, c));
        queue.push_back((r, c));

        while let Some((curr_r, curr_c)) = queue.pop_front() {
            let curr_idx = self.idx(curr_r, curr_c);
            if self.grid[curr_idx].adjacent_mines == 0 {
                for (nr, nc) in self.get_neighbors(curr_r, curr_c) {
                    let n_idx = self.idx(nr, nc);
                    let n_cell = &mut self.grid[n_idx];
                    if n_cell.state == CellState::Hidden || n_cell.state == CellState::Question {
                        n_cell.state = CellState::Revealed;
                        self.revealed_count += 1;
                        newly_revealed.push((nr, nc));
                        if n_cell.adjacent_mines == 0 {
                            queue.push_back((nr, nc));
                        }
                    }
                }
            }
        }

        // Comprobar victoria
        let non_mine_total = self.rows * self.cols - self.total_mines;
        if self.revealed_count == non_mine_total {
            self.game_state = GameState::Won;
            self.end_time = Some(now);
            for cell in &mut self.grid {
                if cell.is_mine {
                    cell.state = CellState::Flagged;
                }
            }
            self.flags_count = self.total_mines;
            return RevealResult::Win(newly_revealed);
        }

        RevealResult::Ok(newly_revealed)
    }

    pub fn toggle_flag(&mut self, r: usize, c: usize) -> FlagAction {
        if self.game_state == GameState::Won || self.game_state == GameState::Lost {
            return FlagAction::None;
        }
        if r >= self.rows || c >= self.cols {
            return FlagAction::None;
        }

        let idx = self.idx(r, c);
        let cell = &mut self.grid[idx];

        match cell.state {
            CellState::Hidden => {
                cell.state = CellState::Flagged;
                self.flags_count += 1;
                FlagAction::Flag
            }
            CellState::Flagged => {
                if self.allow_question {
                    cell.state = CellState::Question;
                    self.flags_count = self.flags_count.saturating_sub(1);
                    FlagAction::Question
                } else {
                    cell.state = CellState::Hidden;
                    self.flags_count = self.flags_count.saturating_sub(1);
                    FlagAction::Unflag
                }
            }
            CellState::Question => {
                cell.state = CellState::Hidden;
                FlagAction::Unflag
            }
            CellState::Revealed => FlagAction::None,
        }
    }

    pub fn chord(&mut self, r: usize, c: usize, now: f64) -> RevealResult {
        if self.game_state != GameState::Playing {
            return RevealResult::None;
        }
        if r >= self.rows || c >= self.cols {
            return RevealResult::None;
        }

        let idx = self.idx(r, c);
        let cell = self.grid[idx];
        if cell.state != CellState::Revealed || cell.adjacent_mines == 0 {
            return RevealResult::None;
        }

        let neighbors = self.get_neighbors(r, c);
        let flag_count = neighbors
            .iter()
            .filter(|&&(nr, nc)| self.grid[self.idx(nr, nc)].state == CellState::Flagged)
            .count() as u8;

        if flag_count != cell.adjacent_mines {
            return RevealResult::None;
        }

        let mut all_revealed = Vec::new();
        let mut hit_mine = None;

        for (nr, nc) in neighbors {
            let n_idx = self.idx(nr, nc);
            let n_state = self.grid[n_idx].state;
            if n_state == CellState::Hidden || n_state == CellState::Question {
                if self.grid[n_idx].is_mine {
                    self.grid[n_idx].state = CellState::Revealed;
                    if hit_mine.is_none() {
                        hit_mine = Some((nr, nc));
                    }
                } else {
                    match self.reveal(nr, nc, now) {
                        RevealResult::Win(rev) => {
                            all_revealed.extend(rev);
                            return RevealResult::Win(all_revealed);
                        }
                        RevealResult::Ok(rev) => {
                            all_revealed.extend(rev);
                        }
                        _ => {}
                    }
                }
            }
        }

        if let Some((mr, mc)) = hit_mine {
            self.game_state = GameState::Lost;
            self.end_time = Some(now);
            self.exploded_cell = Some((mr, mc));
            return RevealResult::Mine(mr, mc);
        }

        if !all_revealed.is_empty() {
            RevealResult::Ok(all_revealed)
        } else {
            RevealResult::None
        }
    }

    pub fn pause(&mut self, now: f64) {
        if self.game_state == GameState::Playing && self.paused_at.is_none() {
            self.paused_at = Some(now);
        }
    }

    pub fn resume(&mut self, now: f64) {
        if self.game_state != GameState::Playing {
            self.paused_at = None;
            return;
        }
        if let Some(paused_at) = self.paused_at.take()
            && let Some(start_time) = &mut self.start_time
        {
            *start_time += (now - paused_at).max(0.0);
        }
    }

    pub fn remaining_mines(&self) -> i32 {
        self.total_mines as i32 - self.flags_count as i32
    }

    pub fn elapsed_seconds(&self, now: f64) -> u32 {
        match self.game_state {
            GameState::Ready => 0,
            GameState::Playing => {
                if let Some(st) = self.start_time {
                    let effective_now = self.paused_at.unwrap_or(now);
                    ((effective_now - st).max(0.0) as u32).min(999)
                } else {
                    0
                }
            }
            GameState::Won | GameState::Lost => {
                if let (Some(st), Some(et)) = (self.start_time, self.end_time) {
                    ((et - st).max(0.0) as u32).min(999)
                } else {
                    0
                }
            }
        }
    }
}

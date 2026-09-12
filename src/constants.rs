use macroquad::prelude::*;

pub const CELL_SIZE: f32 = 32.0;
pub const HEADER_HEIGHT: f32 = 56.0;
pub const MENU_HEIGHT: f32 = 26.0;
pub const MARGIN: f32 = 12.0;
pub const SMILEY_SIZE: f32 = 38.0;

// Paleta clásica de Windows
pub const BG_COLOR: Color = Color::new(192.0 / 255.0, 192.0 / 255.0, 192.0 / 255.0, 1.0);
pub const COLOR_WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
pub const COLOR_LIGHT_GRAY: Color = Color::new(220.0 / 255.0, 220.0 / 255.0, 220.0 / 255.0, 1.0);
pub const COLOR_GRAY: Color = Color::new(192.0 / 255.0, 192.0 / 255.0, 192.0 / 255.0, 1.0);
pub const COLOR_DARK_GRAY: Color = Color::new(128.0 / 255.0, 128.0 / 255.0, 128.0 / 255.0, 1.0);
pub const COLOR_VERY_DARK: Color = Color::new(64.0 / 255.0, 64.0 / 255.0, 64.0 / 255.0, 1.0);
pub const COLOR_BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
pub const COLOR_RED: Color = Color::new(230.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0, 1.0);
pub const COLOR_MINE_EXPLODED_BG: Color =
    Color::new(235.0 / 255.0, 50.0 / 255.0, 50.0 / 255.0, 1.0);

// Colores para displays LED
pub const LED_BG: Color = Color::new(15.0 / 255.0, 0.0, 0.0, 1.0);
pub const LED_ON: Color = Color::new(1.0, 25.0 / 255.0, 25.0 / 255.0, 1.0);
pub const LED_OFF: Color = Color::new(55.0 / 255.0, 10.0 / 255.0, 10.0 / 255.0, 1.0);

pub fn get_number_color(n: u8) -> Color {
    match n {
        1 => Color::new(0.0, 0.0, 230.0 / 255.0, 1.0), // Azul
        2 => Color::new(0.0, 128.0 / 255.0, 0.0, 1.0), // Verde
        3 => Color::new(220.0 / 255.0, 0.0, 0.0, 1.0), // Rojo
        4 => Color::new(0.0, 0.0, 130.0 / 255.0, 1.0), // Azul marino
        5 => Color::new(128.0 / 255.0, 0.0, 0.0, 1.0), // Granate
        6 => Color::new(0.0, 128.0 / 255.0, 128.0 / 255.0, 1.0), // Verde azulado
        7 => Color::new(16.0 / 255.0, 16.0 / 255.0, 16.0 / 255.0, 1.0), // Negro
        8 => Color::new(110.0 / 255.0, 110.0 / 255.0, 110.0 / 255.0, 1.0), // Gris
        _ => COLOR_BLACK,
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CellState {
    Hidden,
    Revealed,
    Flagged,
    Question,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GameState {
    Ready,
    Playing,
    Won,
    Lost,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SmileyState {
    Normal,
    Scared,
    Win,
    Dead,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Difficulty {
    Principiante,
    Intermedio,
    Experto,
    Custom {
        cols: usize,
        rows: usize,
        mines: usize,
    },
}

impl Difficulty {
    pub fn config(&self) -> (usize, usize, usize) {
        match *self {
            Difficulty::Principiante => (9, 9, 10),
            Difficulty::Intermedio => (16, 16, 40),
            Difficulty::Experto => (30, 16, 99),
            Difficulty::Custom { cols, rows, mines } => (cols, rows, mines),
        }
    }

    pub fn id_str(&self) -> &'static str {
        match *self {
            Difficulty::Principiante => "principiante",
            Difficulty::Intermedio => "intermedio",
            Difficulty::Experto => "experto",
            Difficulty::Custom { .. } => "custom",
        }
    }

    pub fn name_str(&self) -> &'static str {
        match *self {
            Difficulty::Principiante => "Principiante",
            Difficulty::Intermedio => "Intermedio",
            Difficulty::Experto => "Experto",
            Difficulty::Custom { .. } => "Personalizado",
        }
    }
}

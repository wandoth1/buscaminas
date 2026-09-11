use macroquad::prelude::*;
use crate::constants::*;
use crate::engine::Cell;

// =========================================================================
// Sistema de Partículas (Confeti y Escombros de Explosión)
// =========================================================================

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub color: Color,
    pub life: f32,
    pub max_life: f32,
    pub size: f32,
}

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self { particles: Vec::new() }
    }

    pub fn spawn_confetti(&mut self, center_x: f32, center_y: f32, count: usize) {
        let colors = [
            Color::new(1.0, 0.2, 0.2, 1.0),
            Color::new(0.2, 1.0, 0.2, 1.0),
            Color::new(0.2, 0.5, 1.0, 1.0),
            Color::new(1.0, 0.9, 0.1, 1.0),
            Color::new(1.0, 0.3, 0.9, 1.0),
            Color::new(0.3, 1.0, 0.9, 1.0),
        ];

        for _ in 0..count {
            let angle = (macroquad::rand::gen_range(0, 360) as f32).to_radians();
            let speed = macroquad::rand::gen_range(80, 350) as f32;
            let c_idx = macroquad::rand::gen_range(0, colors.len() as i32) as usize;
            let life = (macroquad::rand::gen_range(120, 250) as f32) / 100.0;
            self.particles.push(Particle {
                x: center_x + (macroquad::rand::gen_range(-40, 40) as f32),
                y: center_y + (macroquad::rand::gen_range(-20, 20) as f32),
                vx: angle.cos() * speed,
                vy: angle.sin() * speed - 120.0,
                color: colors[c_idx],
                life,
                max_life: life,
                size: (macroquad::rand::gen_range(3, 7) as f32),
            });
        }
    }

    pub fn spawn_explosion(&mut self, center_x: f32, center_y: f32, count: usize) {
        let colors = [
            Color::new(1.0, 0.1, 0.0, 1.0),
            Color::new(1.0, 0.5, 0.0, 1.0),
            Color::new(0.3, 0.3, 0.3, 1.0),
            Color::new(0.1, 0.1, 0.1, 1.0),
        ];

        for _ in 0..count {
            let angle = (macroquad::rand::gen_range(0, 360) as f32).to_radians();
            let speed = macroquad::rand::gen_range(50, 280) as f32;
            let c_idx = macroquad::rand::gen_range(0, colors.len() as i32) as usize;
            let life = (macroquad::rand::gen_range(40, 90) as f32) / 100.0;
            self.particles.push(Particle {
                x: center_x,
                y: center_y,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed,
                color: colors[c_idx],
                life,
                max_life: life,
                size: (macroquad::rand::gen_range(2, 6) as f32),
            });
        }
    }

    pub fn update(&mut self, dt: f32) {
        for p in &mut self.particles {
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.vy += 320.0 * dt; // Gravedad
            p.life -= dt;
        }
        self.particles.retain(|p| p.life > 0.0);
    }

    pub fn draw(&self) {
        for p in &self.particles {
            let alpha = (p.life / p.max_life).clamp(0.0, 1.0);
            let mut col = p.color;
            col.a = alpha;
            draw_rectangle(p.x, p.y, p.size, p.size, col);
        }
    }
}

// =========================================================================
// Funciones de Renderizado Gráfico
// =========================================================================

pub fn draw_sunken_rect(x: f32, y: f32, w: f32, h: f32, border: f32, bg_color: Option<Color>) {
    if let Some(bg) = bg_color {
        draw_rectangle(x, y, w, h, bg);
    }
    for i in 0..(border as usize) {
        let fi = i as f32;
        let c_dark = if i == 0 { COLOR_DARK_GRAY } else { COLOR_VERY_DARK };
        let c_light = COLOR_WHITE;
        // Arriba e Izquierda
        draw_line(x + fi, y + fi, x + w - 1.0 - fi, y + fi, 1.0, c_dark);
        draw_line(x + fi, y + fi, x + fi, y + h - 1.0 - fi, 1.0, c_dark);
        // Abajo y Derecha
        draw_line(x + fi, y + h - 1.0 - fi, x + w - 1.0 - fi, y + h - 1.0 - fi, 1.0, c_light);
        draw_line(x + w - 1.0 - fi, y + fi, x + w - 1.0 - fi, y + h - 1.0 - fi, 1.0, c_light);
    }
}

pub fn draw_raised_rect(x: f32, y: f32, w: f32, h: f32, border: f32, bg_color: Option<Color>) {
    if let Some(bg) = bg_color {
        draw_rectangle(x, y, w, h, bg);
    }
    for i in 0..(border as usize) {
        let fi = i as f32;
        let c_light = COLOR_WHITE;
        let c_dark = if i == 0 { COLOR_DARK_GRAY } else { COLOR_VERY_DARK };
        // Arriba e Izquierda
        draw_line(x + fi, y + fi, x + w - 1.0 - fi, y + fi, 1.0, c_light);
        draw_line(x + fi, y + fi, x + fi, y + h - 1.0 - fi, 1.0, c_light);
        // Abajo y Derecha
        draw_line(x + fi, y + h - 1.0 - fi, x + w - 1.0 - fi, y + h - 1.0 - fi, 1.0, c_dark);
        draw_line(x + w - 1.0 - fi, y + fi, x + w - 1.0 - fi, y + h - 1.0 - fi, 1.0, c_dark);
    }
}

pub fn draw_digit_7seg(digit: char, x: f32, y: f32, w: f32, h: f32, t: f32) {
    let mapping = match digit {
        '0' => "abcdef",
        '1' => "bc",
        '2' => "abdeg",
        '3' => "abcdg",
        '4' => "bcfg",
        '5' => "acdfg",
        '6' => "acdefg",
        '7' => "abc",
        '8' => "abcdefg",
        '9' => "abcdfg",
        '-' => "g",
        _ => "",
    };

    let half_h = (h / 2.0).floor();

    let seg_a = mapping.contains('a');
    let seg_b = mapping.contains('b');
    let seg_c = mapping.contains('c');
    let seg_d = mapping.contains('d');
    let seg_e = mapping.contains('e');
    let seg_f = mapping.contains('f');
    let seg_g = mapping.contains('g');

    let col = |active: bool| if active { LED_ON } else { LED_OFF };

    // Segmento A (superior horizontal)
    draw_rectangle(x + 2.0, y, w - 4.0, t, col(seg_a));
    // Segmento B (superior derecho vertical)
    draw_rectangle(x + w - t, y + 2.0, t, half_h - 2.0, col(seg_b));
    // Segmento C (inferior derecho vertical)
    draw_rectangle(x + w - t, y + half_h, t, h - half_h - 2.0, col(seg_c));
    // Segmento D (inferior horizontal)
    draw_rectangle(x + 2.0, y + h - t, w - 4.0, t, col(seg_d));
    // Segmento E (inferior izquierdo vertical)
    draw_rectangle(x, y + half_h, t, h - half_h - 2.0, col(seg_e));
    // Segmento F (superior izquierdo vertical)
    draw_rectangle(x, y + 2.0, t, half_h - 2.0, col(seg_f));
    // Segmento G (central horizontal)
    draw_rectangle(x + 2.0, y + half_h - (t / 2.0), w - 4.0, t, col(seg_g));
}

pub fn draw_led_counter(value: i32, x: f32, y: f32) {
    let text = if value < -99 {
        "-99".to_string()
    } else if value < 0 {
        format!("-{:02}", value.abs())
    } else if value > 999 {
        "999".to_string()
    } else {
        format!("{:03}", value)
    };

    let box_w = 64.0;
    let box_h = 34.0;
    draw_sunken_rect(x, y, box_w, box_h, 2.0, Some(LED_BG));

    let digit_w = 14.0;
    let digit_h = 26.0;
    let gap = 4.0;
    let start_x = x + (box_w - (3.0 * digit_w + 2.0 * gap)) / 2.0;
    let start_y = y + (box_h - digit_h) / 2.0;

    for (i, ch) in text.chars().enumerate() {
        let dx = start_x + (i as f32) * (digit_w + gap);
        draw_digit_7seg(ch, dx, start_y, digit_w, digit_h, 3.0);
    }
}

pub fn draw_smiley(x: f32, y: f32, size: f32, state: SmileyState, pressed: bool) {
    if pressed {
        draw_sunken_rect(x, y, size, size, 2.0, Some(COLOR_GRAY));
    } else {
        draw_raised_rect(x, y, size, size, 2.0, Some(COLOR_GRAY));
    }

    let offset = if pressed { 1.0 } else { 0.0 };
    let cx = x + size / 2.0 + offset;
    let cy = y + size / 2.0 + offset;
    let radius = 14.0;

    // Círculo amarillo
    draw_circle(cx, cy, radius, Color::new(1.0, 230.0 / 255.0, 20.0 / 255.0, 1.0));
    draw_circle_lines(cx, cy, radius, 1.0, Color::new(50.0 / 255.0, 40.0 / 255.0, 0.0, 1.0));

    match state {
        SmileyState::Normal => {
            // Ojos
            draw_circle(cx - 5.0, cy - 3.0, 2.0, COLOR_BLACK);
            draw_circle(cx + 5.0, cy - 3.0, 2.0, COLOR_BLACK);
            // Sonrisa
            draw_line(cx - 6.0, cy + 3.0, cx - 2.0, cy + 6.0, 1.5, COLOR_BLACK);
            draw_line(cx - 2.0, cy + 6.0, cx + 2.0, cy + 6.0, 1.5, COLOR_BLACK);
            draw_line(cx + 2.0, cy + 6.0, cx + 6.0, cy + 3.0, 1.5, COLOR_BLACK);
        }
        SmileyState::Scared => {
            // Ojos abiertos
            draw_circle_lines(cx - 5.0, cy - 4.0, 3.0, 1.5, COLOR_BLACK);
            draw_circle_lines(cx + 5.0, cy - 4.0, 3.0, 1.5, COLOR_BLACK);
            // Boca redonda
            draw_circle_lines(cx, cy + 4.0, 3.0, 1.5, COLOR_BLACK);
        }
        SmileyState::Win => {
            // Gafas de sol negras
            draw_rectangle(cx - 9.0, cy - 4.0, 7.0, 6.0, COLOR_BLACK);
            draw_rectangle(cx + 2.0, cy - 4.0, 7.0, 6.0, COLOR_BLACK);
            draw_line(cx - 2.0, cy - 3.0, cx + 2.0, cy - 3.0, 2.0, COLOR_BLACK);
            draw_line(cx - 9.0, cy - 3.0, cx - 13.0, cy - 5.0, 1.5, COLOR_BLACK);
            draw_line(cx + 9.0, cy - 3.0, cx + 13.0, cy - 5.0, 1.5, COLOR_BLACK);
            // Brillos blancos
            draw_line(cx - 7.0, cy - 2.0, cx - 4.0, cy + 1.0, 1.0, COLOR_WHITE);
            draw_line(cx + 4.0, cy - 2.0, cx + 7.0, cy + 1.0, 1.0, COLOR_WHITE);
            // Sonrisa
            draw_line(cx - 6.0, cy + 4.0, cx - 2.0, cy + 7.0, 1.5, COLOR_BLACK);
            draw_line(cx - 2.0, cy + 7.0, cx + 2.0, cy + 7.0, 1.5, COLOR_BLACK);
            draw_line(cx + 2.0, cy + 7.0, cx + 6.0, cy + 4.0, 1.5, COLOR_BLACK);
        }
        SmileyState::Dead => {
            // Cruces en ojos
            draw_line(cx - 7.0, cy - 5.0, cx - 3.0, cy - 1.0, 2.0, COLOR_BLACK);
            draw_line(cx - 7.0, cy - 1.0, cx - 3.0, cy - 5.0, 2.0, COLOR_BLACK);
            draw_line(cx + 3.0, cy - 5.0, cx + 7.0, cy - 1.0, 2.0, COLOR_BLACK);
            draw_line(cx + 3.0, cy - 1.0, cx + 7.0, cy - 5.0, 2.0, COLOR_BLACK);
            // Boca triste
            draw_line(cx - 6.0, cy + 6.0, cx - 2.0, cy + 3.0, 1.5, COLOR_BLACK);
            draw_line(cx - 2.0, cy + 3.0, cx + 2.0, cy + 3.0, 1.5, COLOR_BLACK);
            draw_line(cx + 2.0, cy + 3.0, cx + 6.0, cy + 6.0, 1.5, COLOR_BLACK);
        }
    }
}

pub fn draw_mine(cx: f32, cy: f32, exploded: bool) {
    let spikes = [
        ((cx - 10.0, cy), (cx + 10.0, cy)),
        ((cx, cy - 10.0), (cx, cy + 10.0)),
        ((cx - 7.0, cy - 7.0), (cx + 7.0, cy + 7.0)),
        ((cx - 7.0, cy + 7.0), (cx + 7.0, cy - 7.0)),
    ];
    for (p1, p2) in spikes {
        draw_line(p1.0, p1.1, p2.0, p2.1, 2.0, COLOR_BLACK);
    }
    draw_circle(cx, cy, 6.0, COLOR_BLACK);
    draw_circle(cx - 2.0, cy - 2.0, 2.0, COLOR_WHITE);

    let core_col = if exploded { Color::new(1.0, 0.2, 0.2, 1.0) } else { Color::new(0.7, 0.1, 0.1, 1.0) };
    draw_circle(cx + 2.0, cy + 2.0, 1.0, core_col);
}

pub fn draw_flag(cx: f32, cy: f32) {
    draw_rectangle(cx - 7.0, cy + 8.0, 14.0, 2.0, COLOR_BLACK);
    draw_rectangle(cx - 5.0, cy + 6.0, 10.0, 2.0, COLOR_VERY_DARK);
    draw_rectangle(cx - 3.0, cy + 4.0, 6.0, 2.0, COLOR_DARK_GRAY);

    draw_line(cx - 2.0, cy - 9.0, cx - 2.0, cy + 6.0, 2.0, COLOR_BLACK);
    draw_circle(cx - 2.0, cy - 9.0, 1.0, Color::new(0.9, 0.75, 0.15, 1.0));

    // Bandera triangular roja
    draw_triangle(
        Vec2::new(cx - 1.0, cy - 9.0),
        Vec2::new(cx + 9.0, cy - 4.0),
        Vec2::new(cx - 1.0, cy + 1.0),
        COLOR_RED,
    );
}

pub fn draw_false_flag(cx: f32, cy: f32) {
    draw_mine(cx, cy, false);
    draw_line(cx - 10.0, cy - 10.0, cx + 10.0, cy + 10.0, 3.0, COLOR_RED);
    draw_line(cx - 10.0, cy + 10.0, cx + 10.0, cy - 10.0, 3.0, COLOR_RED);
}

pub fn draw_cell(
    x: f32,
    y: f32,
    cell: &Cell,
    is_exploded: bool,
    is_chord_pressed: bool,
    game_lost: bool,
) {
    let cx = x + CELL_SIZE / 2.0;
    let cy = y + CELL_SIZE / 2.0;

    match cell.state {
        CellState::Hidden => {
            if is_chord_pressed {
                draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, COLOR_GRAY);
                draw_rectangle_lines(x, y, CELL_SIZE, CELL_SIZE, 1.0, COLOR_DARK_GRAY);
            } else if game_lost && cell.is_mine {
                draw_sunken_rect(x, y, CELL_SIZE, CELL_SIZE, 1.0, Some(COLOR_GRAY));
                draw_mine(cx, cy, false);
            } else {
                draw_raised_rect(x, y, CELL_SIZE, CELL_SIZE, 3.0, Some(COLOR_GRAY));
            }
        }
        CellState::Flagged => {
            draw_raised_rect(x, y, CELL_SIZE, CELL_SIZE, 3.0, Some(COLOR_GRAY));
            if game_lost && !cell.is_mine {
                draw_false_flag(cx, cy);
            } else {
                draw_flag(cx, cy);
            }
        }
        CellState::Question => {
            if is_chord_pressed {
                draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, COLOR_GRAY);
                draw_rectangle_lines(x, y, CELL_SIZE, CELL_SIZE, 1.0, COLOR_DARK_GRAY);
            } else if game_lost && cell.is_mine {
                draw_sunken_rect(x, y, CELL_SIZE, CELL_SIZE, 1.0, Some(COLOR_GRAY));
                draw_mine(cx, cy, false);
            } else {
                draw_raised_rect(x, y, CELL_SIZE, CELL_SIZE, 3.0, Some(COLOR_GRAY));
                draw_text("?", cx - 5.0, cy + 7.0, 24.0, Color::new(0.0, 0.0, 0.6, 1.0));
            }
        }
        CellState::Revealed => {
            if cell.is_mine {
                let bg = if is_exploded { COLOR_MINE_EXPLODED_BG } else { COLOR_GRAY };
                draw_sunken_rect(x, y, CELL_SIZE, CELL_SIZE, 1.0, Some(bg));
                draw_mine(cx, cy, is_exploded);
            } else {
                draw_sunken_rect(x, y, CELL_SIZE, CELL_SIZE, 1.0, Some(COLOR_GRAY));
                if cell.adjacent_mines > 0 {
                    let col = get_number_color(cell.adjacent_mines);
                    let num_str = cell.adjacent_mines.to_string();
                    let font_size = 24.0;
                    let text_dim = measure_text(&num_str, None, font_size as u16, 1.0);
                    draw_text(
                        &num_str,
                        cx - text_dim.width / 2.0,
                        cy + text_dim.height / 2.0 - 1.0,
                        font_size,
                        col,
                    );
                }
            }
        }
    }
}

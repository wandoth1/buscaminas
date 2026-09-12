#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod constants;
mod engine;
mod highscores;
mod render;
mod sound;
mod ui;

use constants::*;
use engine::{Board, FlagAction, RevealResult};
use highscores::HighScoreManager;
use macroquad::prelude::*;
use render::*;
use sound::SoundManager;
use ui::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Buscaminas v2.2.2 (Rust)".to_string(),
        window_width: 312,
        window_height: 396,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut current_diff = Difficulty::Principiante;
    let (init_cols, init_rows, init_mines) = current_diff.config();

    let mut board = Board::new(init_cols, init_rows, init_mines);
    let mut sound = SoundManager::new().await;
    let mut highscores = HighScoreManager::new();
    let mut menu_bar = MenuBar::new();
    let mut dialog = DialogState::None;
    let mut particles = ParticleSystem::new();
    let mut particles_enabled = true;

    let mut mouse_left_down = false;
    let mut mouse_right_down = false;
    let mut mouse_middle_down = false;
    let mut smiley_pressed = false;
    let mut shake_timer = 0.0f32;

    // Ajuste de tamaño inicial
    let mut target_window_w = (board.cols as f32 * CELL_SIZE + 2.0 * MARGIN).max(260.0);
    let mut target_window_h =
        MENU_HEIGHT + HEADER_HEIGHT + (board.rows as f32 * CELL_SIZE) + 3.0 * MARGIN;
    request_new_screen_size(target_window_w, target_window_h);

    loop {
        let dt = get_frame_time();
        let now = get_time();
        let screen_w = screen_width();
        let screen_h = screen_height();

        // -------------------------------------------------------------
        // Actualizar sacudida de pantalla (Screen Shake) y partículas
        // -------------------------------------------------------------
        if shake_timer > 0.0 {
            shake_timer -= dt;
        }
        if particles_enabled {
            particles.update(dt);
        }

        // Posiciones del layout
        let board_w = board.cols as f32 * CELL_SIZE;
        let board_h = board.rows as f32 * CELL_SIZE;
        let board_x = ((screen_w - board_w) / 2.0).floor();
        let board_y = MENU_HEIGHT + HEADER_HEIGHT + MARGIN;

        let header_x = MARGIN;
        let header_y = MENU_HEIGHT + MARGIN;
        let header_w = screen_w - 2.0 * MARGIN;
        let header_h = HEADER_HEIGHT - MARGIN;

        let sm_x = header_x + (header_w - SMILEY_SIZE) / 2.0;
        let sm_y = header_y + (header_h - SMILEY_SIZE) / 2.0;
        let smiley_rect = Rect::new(sm_x, sm_y, SMILEY_SIZE, SMILEY_SIZE);

        let mouse_pos = Vec2::from(mouse_position());

        // Función auxiliar para celda bajo el cursor
        let hovered_cell = if mouse_pos.x >= board_x
            && mouse_pos.x < board_x + board_w
            && mouse_pos.y >= board_y
            && mouse_pos.y < board_y + board_h
        {
            let c = ((mouse_pos.x - board_x) / CELL_SIZE) as usize;
            let r = ((mouse_pos.y - board_y) / CELL_SIZE) as usize;
            if r < board.rows && c < board.cols {
                Some((r, c))
            } else {
                None
            }
        } else {
            None
        };

        // -------------------------------------------------------------
        // Procesamiento de Eventos de Diálogo y Menú
        // -------------------------------------------------------------
        let dialog_active = !matches!(dialog, DialogState::None);

        if dialog_active {
            board.pause(now);
            mouse_left_down = false;
            mouse_right_down = false;
            mouse_middle_down = false;
            smiley_pressed = false;

            match draw_dialog(&mut dialog, &mut highscores, screen_w, screen_h) {
                DialogEvent::ApplyCustom(c, r, m) => {
                    current_diff = Difficulty::Custom {
                        cols: c,
                        rows: r,
                        mines: m,
                    };
                    board.reset(c, r, m);
                    target_window_w = (c as f32 * CELL_SIZE + 2.0 * MARGIN).max(260.0);
                    target_window_h =
                        MENU_HEIGHT + HEADER_HEIGHT + (r as f32 * CELL_SIZE) + 3.0 * MARGIN;
                    request_new_screen_size(target_window_w, target_window_h);
                }
                DialogEvent::SaveRecord(d_id, name, secs) => {
                    highscores.add_score(&d_id, name, secs);
                    let tab = match d_id.as_str() {
                        "intermedio" => "intermedio",
                        "experto" => "experto",
                        _ => "principiante",
                    };
                    dialog = DialogState::HighScores { tab };
                }
                DialogEvent::ResetHighScores => {}
                DialogEvent::Close | DialogEvent::None => {}
            }
            next_frame().await;
            continue;
        }

        board.resume(now);

        // Barra de Menús
        let menu_was_open = menu_bar.active_menu.is_some();
        let menu_action = menu_bar.draw(
            screen_w,
            current_diff,
            board.allow_question,
            sound.enabled,
            particles_enabled,
            true,
        );

        if let Some(act) = menu_action {
            match act {
                MenuAction::NewGame => {
                    let (c, r, m) = current_diff.config();
                    board.reset(c, r, m);
                }
                MenuAction::DiffPrincipiante => {
                    current_diff = Difficulty::Principiante;
                    let (c, r, m) = current_diff.config();
                    board.reset(c, r, m);
                    target_window_w = (c as f32 * CELL_SIZE + 2.0 * MARGIN).max(260.0);
                    target_window_h =
                        MENU_HEIGHT + HEADER_HEIGHT + (r as f32 * CELL_SIZE) + 3.0 * MARGIN;
                    request_new_screen_size(target_window_w, target_window_h);
                }
                MenuAction::DiffIntermedio => {
                    current_diff = Difficulty::Intermedio;
                    let (c, r, m) = current_diff.config();
                    board.reset(c, r, m);
                    target_window_w = (c as f32 * CELL_SIZE + 2.0 * MARGIN).max(260.0);
                    target_window_h =
                        MENU_HEIGHT + HEADER_HEIGHT + (r as f32 * CELL_SIZE) + 3.0 * MARGIN;
                    request_new_screen_size(target_window_w, target_window_h);
                }
                MenuAction::DiffExperto => {
                    current_diff = Difficulty::Experto;
                    let (c, r, m) = current_diff.config();
                    board.reset(c, r, m);
                    target_window_w = (c as f32 * CELL_SIZE + 2.0 * MARGIN).max(260.0);
                    target_window_h =
                        MENU_HEIGHT + HEADER_HEIGHT + (r as f32 * CELL_SIZE) + 3.0 * MARGIN;
                    request_new_screen_size(target_window_w, target_window_h);
                }
                MenuAction::DiffCustom => {
                    dialog = DialogState::Custom {
                        cols: board.cols,
                        rows: board.rows,
                        mines: board.total_mines,
                    };
                }
                MenuAction::OpenRecords => {
                    let tab = match current_diff {
                        Difficulty::Intermedio => "intermedio",
                        Difficulty::Experto => "experto",
                        _ => "principiante",
                    };
                    dialog = DialogState::HighScores { tab };
                }
                MenuAction::ToggleMarks => {
                    board.allow_question = !board.allow_question;
                }
                MenuAction::ToggleSound => {
                    sound.toggle();
                }
                MenuAction::ToggleParticles => {
                    particles_enabled = !particles_enabled;
                }
                MenuAction::OpenHelp => {
                    dialog = DialogState::Help;
                }
                MenuAction::OpenAbout => {
                    dialog = DialogState::About;
                }
                MenuAction::Exit => {
                    std::process::exit(0);
                }
            }
        }

        // Si hay un menú desplegado, omitir interacción con el tablero
        let menu_open = menu_bar.active_menu.is_some();
        let board_input_blocked = menu_was_open || menu_open;

        // -------------------------------------------------------------
        // Atajos de teclado rápidos
        // -------------------------------------------------------------
        if !board_input_blocked {
            if is_key_pressed(KeyCode::F2) {
                let (c, r, m) = current_diff.config();
                board.reset(c, r, m);
            } else if is_key_pressed(KeyCode::Key1) {
                current_diff = Difficulty::Principiante;
                let (c, r, m) = current_diff.config();
                board.reset(c, r, m);
                target_window_w = (c as f32 * CELL_SIZE + 2.0 * MARGIN).max(260.0);
                target_window_h =
                    MENU_HEIGHT + HEADER_HEIGHT + (r as f32 * CELL_SIZE) + 3.0 * MARGIN;
                request_new_screen_size(target_window_w, target_window_h);
            } else if is_key_pressed(KeyCode::Key2) {
                current_diff = Difficulty::Intermedio;
                let (c, r, m) = current_diff.config();
                board.reset(c, r, m);
                target_window_w = (c as f32 * CELL_SIZE + 2.0 * MARGIN).max(260.0);
                target_window_h =
                    MENU_HEIGHT + HEADER_HEIGHT + (r as f32 * CELL_SIZE) + 3.0 * MARGIN;
                request_new_screen_size(target_window_w, target_window_h);
            } else if is_key_pressed(KeyCode::Key3) {
                current_diff = Difficulty::Experto;
                let (c, r, m) = current_diff.config();
                board.reset(c, r, m);
                target_window_w = (c as f32 * CELL_SIZE + 2.0 * MARGIN).max(260.0);
                target_window_h =
                    MENU_HEIGHT + HEADER_HEIGHT + (r as f32 * CELL_SIZE) + 3.0 * MARGIN;
                request_new_screen_size(target_window_w, target_window_h);
            } else if is_key_pressed(KeyCode::M) {
                sound.toggle();
            } else if is_key_pressed(KeyCode::P) {
                particles_enabled = !particles_enabled;
            }
        }

        // -------------------------------------------------------------
        // Interacción del Ratón
        // -------------------------------------------------------------
        if !board_input_blocked {
            if is_mouse_button_pressed(MouseButton::Left) {
                mouse_left_down = true;
                if smiley_rect.contains(mouse_pos) {
                    smiley_pressed = true;
                }
            }
            if is_mouse_button_pressed(MouseButton::Right) {
                mouse_right_down = true;
                if let Some((r, c)) = hovered_cell
                    && !mouse_left_down
                {
                    match board.toggle_flag(r, c) {
                        FlagAction::Flag => sound.play_flag(),
                        FlagAction::Unflag | FlagAction::Question => sound.play_unflag(),
                        FlagAction::None => {}
                    }
                }
            }
            if is_mouse_button_pressed(MouseButton::Middle) {
                mouse_middle_down = true;
            }

            // Soltar botones. Exigimos que la pulsacion se haya registrado aqui:
            // si el press se consumio en el menu o en un dialogo, el release
            // posterior no debe revelar la casilla que haya bajo el cursor.
            if is_mouse_button_released(MouseButton::Left) && mouse_left_down {
                if smiley_pressed {
                    if smiley_rect.contains(mouse_pos) {
                        let (c, r, m) = current_diff.config();
                        board.reset(c, r, m);
                    }
                    smiley_pressed = false;
                }

                let is_chord = mouse_middle_down || mouse_right_down;
                if is_chord {
                    if let Some((r, c)) = hovered_cell {
                        match board.chord(r, c, now) {
                            RevealResult::Mine(mr, mc) => {
                                sound.play_lose();
                                shake_timer = 0.35;
                                if particles_enabled {
                                    let cx = board_x + mc as f32 * CELL_SIZE + CELL_SIZE / 2.0;
                                    let cy = board_y + mr as f32 * CELL_SIZE + CELL_SIZE / 2.0;
                                    particles.spawn_explosion(cx, cy, 35);
                                }
                            }
                            RevealResult::Win(_) => {
                                sound.play_win();
                                if particles_enabled {
                                    particles.spawn_confetti(screen_w / 2.0, board_y + 20.0, 70);
                                }
                                check_and_prompt_record(
                                    &board,
                                    current_diff,
                                    &highscores,
                                    &mut dialog,
                                );
                            }
                            RevealResult::Ok(_) => sound.play_chord(),
                            RevealResult::None => {}
                        }
                    }
                } else if let Some((r, c)) = hovered_cell {
                    let cell = board.grid[board.idx(r, c)];
                    if matches!(cell.state, CellState::Hidden | CellState::Question) {
                        match board.reveal(r, c, now) {
                            RevealResult::Mine(mr, mc) => {
                                sound.play_lose();
                                shake_timer = 0.35;
                                if particles_enabled {
                                    let cx = board_x + mc as f32 * CELL_SIZE + CELL_SIZE / 2.0;
                                    let cy = board_y + mr as f32 * CELL_SIZE + CELL_SIZE / 2.0;
                                    particles.spawn_explosion(cx, cy, 35);
                                }
                            }
                            RevealResult::Win(_) => {
                                sound.play_win();
                                if particles_enabled {
                                    particles.spawn_confetti(screen_w / 2.0, board_y + 20.0, 70);
                                }
                                check_and_prompt_record(
                                    &board,
                                    current_diff,
                                    &highscores,
                                    &mut dialog,
                                );
                            }
                            RevealResult::Ok(_) => sound.play_click(),
                            RevealResult::None => {}
                        }
                    } else if cell.state == CellState::Revealed && cell.adjacent_mines > 0 {
                        match board.chord(r, c, now) {
                            RevealResult::Mine(mr, mc) => {
                                sound.play_lose();
                                shake_timer = 0.35;
                                if particles_enabled {
                                    let cx = board_x + mc as f32 * CELL_SIZE + CELL_SIZE / 2.0;
                                    let cy = board_y + mr as f32 * CELL_SIZE + CELL_SIZE / 2.0;
                                    particles.spawn_explosion(cx, cy, 35);
                                }
                            }
                            RevealResult::Win(_) => {
                                sound.play_win();
                                if particles_enabled {
                                    particles.spawn_confetti(screen_w / 2.0, board_y + 20.0, 70);
                                }
                                check_and_prompt_record(
                                    &board,
                                    current_diff,
                                    &highscores,
                                    &mut dialog,
                                );
                            }
                            RevealResult::Ok(_) => sound.play_chord(),
                            RevealResult::None => {}
                        }
                    }
                }
            }

            if is_mouse_button_released(MouseButton::Middle)
                && mouse_middle_down
                && let Some((r, c)) = hovered_cell
            {
                match board.chord(r, c, now) {
                    RevealResult::Mine(mr, mc) => {
                        sound.play_lose();
                        shake_timer = 0.35;
                        if particles_enabled {
                            let cx = board_x + mc as f32 * CELL_SIZE + CELL_SIZE / 2.0;
                            let cy = board_y + mr as f32 * CELL_SIZE + CELL_SIZE / 2.0;
                            particles.spawn_explosion(cx, cy, 35);
                        }
                    }
                    RevealResult::Win(_) => {
                        sound.play_win();
                        if particles_enabled {
                            particles.spawn_confetti(screen_w / 2.0, board_y + 20.0, 70);
                        }
                        check_and_prompt_record(&board, current_diff, &highscores, &mut dialog);
                    }
                    RevealResult::Ok(_) => sound.play_chord(),
                    RevealResult::None => {}
                }
            }
        }

        // Las transiciones de "soltar" se registran siempre, aunque el tablero
        // este bloqueado: un boton soltado sobre el desplegable no debe quedarse
        // marcado como pulsado, porque falsearia el chording posterior y volveria
        // a dar por valido un release sin pulsacion propia.
        if is_mouse_button_released(MouseButton::Left) {
            mouse_left_down = false;
            smiley_pressed = false;
        }
        if is_mouse_button_released(MouseButton::Right) {
            mouse_right_down = false;
        }
        if is_mouse_button_released(MouseButton::Middle) {
            mouse_middle_down = false;
        }

        // Celdas presionadas para chording
        let is_chording = mouse_middle_down || (mouse_left_down && mouse_right_down);
        let chord_neighbors = if is_chording {
            if let Some((hr, hc)) = hovered_cell {
                let cell = board.grid[board.idx(hr, hc)];
                if cell.state == CellState::Revealed && cell.adjacent_mines > 0 {
                    board.get_neighbors(hr, hc)
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        // -------------------------------------------------------------
        // Renderizado
        // -------------------------------------------------------------
        clear_background(BG_COLOR);

        // Desplazamiento por temblor
        let (shake_x, shake_y) = if shake_timer > 0.0 {
            (
                (macroquad::rand::gen_range(-4, 4) as f32) * (shake_timer / 0.35),
                (macroquad::rand::gen_range(-4, 4) as f32) * (shake_timer / 0.35),
            )
        } else {
            (0.0, 0.0)
        };

        // Marco exterior
        draw_raised_rect(
            0.0,
            MENU_HEIGHT,
            screen_w,
            screen_h - MENU_HEIGHT,
            3.0,
            None,
        );

        // Encabezado
        draw_sunken_rect(header_x, header_y, header_w, header_h, 2.0, Some(BG_COLOR));

        // Contador LED de minas (Izquierda)
        let rem_mines = board.remaining_mines();
        draw_led_counter(
            rem_mines,
            header_x + 8.0,
            header_y + (header_h - 34.0) / 2.0,
        );

        // Smiley (Centro)
        let smiley_state = match board.game_state {
            GameState::Lost => SmileyState::Dead,
            GameState::Won => SmileyState::Win,
            _ => {
                if (mouse_left_down || mouse_middle_down) && hovered_cell.is_some() {
                    SmileyState::Scared
                } else {
                    SmileyState::Normal
                }
            }
        };
        draw_smiley(
            smiley_rect.x,
            smiley_rect.y,
            SMILEY_SIZE,
            smiley_state,
            smiley_pressed,
        );

        // Contador LED de tiempo (Derecha)
        let elapsed = board.elapsed_seconds(now);
        draw_led_counter(
            elapsed as i32,
            header_x + header_w - 64.0 - 8.0,
            header_y + (header_h - 34.0) / 2.0,
        );

        // Tablero con borde hundido
        draw_sunken_rect(
            board_x - 3.0 + shake_x,
            board_y - 3.0 + shake_y,
            board_w + 6.0,
            board_h + 6.0,
            3.0,
            Some(BG_COLOR),
        );

        // Dibujar celdas
        let game_lost = board.game_state == GameState::Lost;
        let exploded_cell = board.exploded_cell;

        for r in 0..board.rows {
            for c in 0..board.cols {
                let cell = &board.grid[board.idx(r, c)];
                let cx_pos = board_x + (c as f32) * CELL_SIZE + shake_x;
                let cy_pos = board_y + (r as f32) * CELL_SIZE + shake_y;

                let is_hovered = hovered_cell == Some((r, c));
                let is_pressed = (is_hovered
                    && mouse_left_down
                    && !mouse_right_down
                    && matches!(cell.state, CellState::Hidden | CellState::Question))
                    || (chord_neighbors.contains(&(r, c))
                        && (cell.state == CellState::Hidden || cell.state == CellState::Question));

                let is_expl = game_lost && exploded_cell == Some((r, c));

                draw_cell(cx_pos, cy_pos, cell, is_expl, is_pressed, game_lost);
            }
        }

        // Partículas
        if particles_enabled {
            particles.draw();
        }

        // Barra de Menús dibujada al final (en capa superior)
        menu_bar.draw(
            screen_w,
            current_diff,
            board.allow_question,
            sound.enabled,
            particles_enabled,
            false,
        );

        next_frame().await;
    }
}

fn check_and_prompt_record(
    board: &Board,
    current_diff: Difficulty,
    highscores: &HighScoreManager,
    dialog: &mut DialogState,
) {
    if matches!(current_diff, Difficulty::Custom { .. }) {
        return;
    }
    let diff_id = current_diff.id_str();
    let elapsed = board.elapsed_seconds(get_time());
    if highscores.is_record(diff_id, elapsed) {
        *dialog = DialogState::NewRecord {
            diff_name: current_diff.name_str().to_string(),
            diff_id: diff_id.to_string(),
            seconds: elapsed,
            name: "Jugador 1".to_string(),
        };
    }
}

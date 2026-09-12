use crate::constants::*;
use crate::highscores::HighScoreManager;
use crate::music::MusicTrack;
use crate::render::{draw_raised_rect, draw_sunken_rect};
use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MenuAction {
    NewGame,
    DiffPrincipiante,
    DiffIntermedio,
    DiffExperto,
    DiffCustom,
    OpenRecords,
    ToggleMarks,
    ToggleSound,
    ToggleParticles,
    MusicRelax,
    MusicFocus,
    MusicOff,
    OpenHelp,
    OpenAbout,
    Exit,
}

/// Lo que el menú necesita saber para dibujarse: dificultad activa y el estado
/// de cada interruptor. Agruparlo evita arrastrar media docena de booleanos por
/// las firmas de `draw` y `draw_dropdown`.
#[derive(Clone, Copy, Debug)]
pub struct MenuState {
    pub current_diff: Difficulty,
    pub allow_question: bool,
    pub sound_enabled: bool,
    pub particles_enabled: bool,
    /// Tema de fondo seleccionado, o `None` si no suena ninguno.
    pub music: Option<MusicTrack>,
}

pub struct MenuBar {
    pub active_menu: Option<usize>,
    input_consumed: bool,
}

impl MenuBar {
    pub fn new() -> Self {
        Self {
            active_menu: None,
            input_consumed: false,
        }
    }

    pub fn draw(&mut self, width: f32, state: MenuState, interactive: bool) -> Option<MenuAction> {
        let mut action = None;
        let mouse_pos = Vec2::from(mouse_position());
        let raw_mouse_pressed = interactive && is_mouse_button_pressed(MouseButton::Left);
        if interactive && !raw_mouse_pressed {
            self.input_consumed = false;
        }
        let mouse_pressed = raw_mouse_pressed && !self.input_consumed;
        if mouse_pressed {
            self.input_consumed = true;
        }

        draw_rectangle(0.0, 0.0, width, MENU_HEIGHT, COLOR_GRAY);
        draw_line(
            0.0,
            MENU_HEIGHT - 1.0,
            width,
            MENU_HEIGHT - 1.0,
            1.0,
            COLOR_DARK_GRAY,
        );

        let menu_titles = ["Juego", "Opciones", "Ayuda"];
        let mut x_offset = 8.0;
        let mut clicked_any_title = false;

        for (idx, title) in menu_titles.iter().enumerate() {
            let dim = measure_text(title, None, 14, 1.0);
            let item_w = dim.width + 16.0;
            let rect = Rect::new(x_offset, 2.0, item_w, MENU_HEIGHT - 4.0);

            let is_open = self.active_menu == Some(idx);
            let is_hover = rect.contains(mouse_pos);

            if interactive && is_hover && self.active_menu.is_some() && !is_open {
                self.active_menu = Some(idx);
            }

            if is_hover && mouse_pressed {
                self.active_menu = if is_open { None } else { Some(idx) };
                clicked_any_title = true;
            }

            if is_open {
                draw_sunken_rect(rect.x, rect.y, rect.w, rect.h, 1.0, Some(COLOR_GRAY));
            } else if is_hover && self.active_menu.is_some() {
                draw_raised_rect(rect.x, rect.y, rect.w, rect.h, 1.0, Some(COLOR_GRAY));
            }

            draw_text(title, x_offset + 8.0, 17.0, 14.0, COLOR_BLACK);
            x_offset += item_w + 4.0;
        }

        if let Some(menu_idx) = self.active_menu {
            let drop_action = self.draw_dropdown(menu_idx, width, state, mouse_pos, mouse_pressed);
            if drop_action.is_some() {
                action = drop_action;
                self.active_menu = None;
            } else if mouse_pressed && !clicked_any_title {
                self.active_menu = None;
            }
        }

        action
    }

    fn draw_dropdown(
        &self,
        menu_idx: usize,
        screen_w: f32,
        state: MenuState,
        mouse_pos: Vec2,
        mouse_pressed: bool,
    ) -> Option<MenuAction> {
        let mut selected_action = None;

        struct Item {
            action: Option<MenuAction>,
            label: &'static str,
            shortcut: &'static str,
            checked: bool,
            separator: bool,
        }

        let items: Vec<Item> = match menu_idx {
            0 => vec![
                Item {
                    action: Some(MenuAction::NewGame),
                    label: "Nuevo",
                    shortcut: "F2",
                    checked: false,
                    separator: false,
                },
                Item {
                    action: None,
                    label: "",
                    shortcut: "",
                    checked: false,
                    separator: true,
                },
                Item {
                    action: Some(MenuAction::DiffPrincipiante),
                    label: "Principiante (9x9)",
                    shortcut: "1",
                    checked: matches!(state.current_diff, Difficulty::Principiante),
                    separator: false,
                },
                Item {
                    action: Some(MenuAction::DiffIntermedio),
                    label: "Intermedio (16x16)",
                    shortcut: "2",
                    checked: matches!(state.current_diff, Difficulty::Intermedio),
                    separator: false,
                },
                Item {
                    action: Some(MenuAction::DiffExperto),
                    label: "Experto (30x16)",
                    shortcut: "3",
                    checked: matches!(state.current_diff, Difficulty::Experto),
                    separator: false,
                },
                Item {
                    action: Some(MenuAction::DiffCustom),
                    label: "Personalizado...",
                    shortcut: "",
                    checked: matches!(state.current_diff, Difficulty::Custom { .. }),
                    separator: false,
                },
                Item {
                    action: None,
                    label: "",
                    shortcut: "",
                    checked: false,
                    separator: true,
                },
                Item {
                    action: Some(MenuAction::OpenRecords),
                    label: "Mejores tiempos...",
                    shortcut: "",
                    checked: false,
                    separator: false,
                },
                Item {
                    action: None,
                    label: "",
                    shortcut: "",
                    checked: false,
                    separator: true,
                },
                Item {
                    action: Some(MenuAction::Exit),
                    label: "Salir",
                    shortcut: "",
                    checked: false,
                    separator: false,
                },
            ],
            1 => vec![
                Item {
                    action: Some(MenuAction::ToggleMarks),
                    label: "Marcas (?) activadas",
                    shortcut: "",
                    checked: state.allow_question,
                    separator: false,
                },
                Item {
                    action: Some(MenuAction::ToggleSound),
                    label: "Sonido activado",
                    shortcut: "M",
                    checked: state.sound_enabled,
                    separator: false,
                },
                Item {
                    action: Some(MenuAction::ToggleParticles),
                    label: "Efectos y partículas",
                    shortcut: "P",
                    checked: state.particles_enabled,
                    separator: false,
                },
                Item {
                    action: None,
                    label: "",
                    shortcut: "",
                    checked: false,
                    separator: true,
                },
                Item {
                    action: Some(MenuAction::MusicRelax),
                    label: "Música: relax",
                    shortcut: "",
                    checked: state.music == Some(MusicTrack::Relax),
                    separator: false,
                },
                Item {
                    action: Some(MenuAction::MusicFocus),
                    label: "Música: concentración",
                    shortcut: "",
                    checked: state.music == Some(MusicTrack::Focus),
                    separator: false,
                },
                Item {
                    action: Some(MenuAction::MusicOff),
                    label: "Música: desactivada",
                    shortcut: "",
                    checked: state.music.is_none(),
                    separator: false,
                },
            ],
            2 => vec![
                Item {
                    action: Some(MenuAction::OpenHelp),
                    label: "Instrucciones y reglas",
                    shortcut: "",
                    checked: false,
                    separator: false,
                },
                Item {
                    action: None,
                    label: "",
                    shortcut: "",
                    checked: false,
                    separator: true,
                },
                Item {
                    action: Some(MenuAction::OpenAbout),
                    label: "Acerca de Buscaminas v2",
                    shortcut: "",
                    checked: false,
                    separator: false,
                },
            ],
            _ => vec![],
        };

        let drop_x = match menu_idx {
            0 => 8.0f32,
            1 => 60.0f32,
            _ => 130.0f32,
        }
        .min(screen_w - 220.0);
        let drop_y = MENU_HEIGHT;
        let drop_w = 215.0;
        let item_h = 24.0;

        let total_h: f32 = items
            .iter()
            .map(|it| if it.separator { 8.0 } else { item_h })
            .sum::<f32>()
            + 6.0;
        draw_raised_rect(drop_x, drop_y, drop_w, total_h, 2.0, Some(COLOR_GRAY));

        let mut curr_y = drop_y + 3.0;

        for item in items {
            if item.separator {
                draw_line(
                    drop_x + 4.0,
                    curr_y + 3.0,
                    drop_x + drop_w - 4.0,
                    curr_y + 3.0,
                    1.0,
                    COLOR_DARK_GRAY,
                );
                draw_line(
                    drop_x + 4.0,
                    curr_y + 4.0,
                    drop_x + drop_w - 4.0,
                    curr_y + 4.0,
                    1.0,
                    COLOR_WHITE,
                );
                curr_y += 8.0;
                continue;
            }

            let item_rect = Rect::new(drop_x + 3.0, curr_y, drop_w - 6.0, item_h);
            let is_hover = item_rect.contains(mouse_pos);

            if is_hover {
                draw_rectangle(
                    item_rect.x,
                    item_rect.y,
                    item_rect.w,
                    item_rect.h,
                    Color::new(0.0, 0.0, 128.0 / 255.0, 1.0),
                );
                if mouse_pressed {
                    selected_action = item.action;
                }
            }

            let text_col = if is_hover { COLOR_WHITE } else { COLOR_BLACK };

            if item.checked {
                let cx = drop_x + 12.0;
                let cy = curr_y + 12.0;
                let check_col = if is_hover { COLOR_WHITE } else { COLOR_BLACK };
                draw_line(cx - 4.0, cy, cx - 1.0, cy + 3.0, 2.0, check_col);
                draw_line(cx - 1.0, cy + 3.0, cx + 5.0, cy - 4.0, 2.0, check_col);
            }

            draw_text(item.label, drop_x + 24.0, curr_y + 16.0, 14.0, text_col);
            if !item.shortcut.is_empty() {
                let s_dim = measure_text(item.shortcut, None, 13, 1.0);
                let s_col = if is_hover {
                    COLOR_LIGHT_GRAY
                } else {
                    COLOR_DARK_GRAY
                };
                draw_text(
                    item.shortcut,
                    drop_x + drop_w - s_dim.width - 12.0,
                    curr_y + 16.0,
                    13.0,
                    s_col,
                );
            }

            curr_y += item_h;
        }

        selected_action
    }
}

pub enum DialogState {
    None,
    Custom {
        cols: usize,
        rows: usize,
        mines: usize,
    },
    HighScores {
        tab: &'static str,
    },
    NewRecord {
        diff_name: String,
        diff_id: String,
        seconds: u32,
        name: String,
    },
    Help,
    About,
}

pub enum DialogEvent {
    None,
    Close,
    ApplyCustom(usize, usize, usize),
    SaveRecord(String, String, u32),
    ResetHighScores,
}

pub fn draw_dialog(
    dialog: &mut DialogState,
    highscores: &mut HighScoreManager,
    screen_w: f32,
    screen_h: f32,
) -> DialogEvent {
    if matches!(dialog, DialogState::None) {
        return DialogEvent::None;
    }

    draw_rectangle(
        0.0,
        0.0,
        screen_w,
        screen_h,
        Color::new(0.0, 0.0, 0.0, 0.45),
    );

    let mouse_pos = Vec2::from(mouse_position());
    let mouse_pressed = is_mouse_button_pressed(MouseButton::Left);

    let draw_window = |title: &str, dw: f32, dh: f32| -> (f32, f32, Rect) {
        let dx = ((screen_w - dw) / 2.0).floor();
        let dy = ((screen_h - dh) / 2.0).floor();
        draw_raised_rect(dx, dy, dw, dh, 3.0, Some(COLOR_GRAY));

        draw_rectangle(
            dx + 3.0,
            dy + 3.0,
            dw - 6.0,
            22.0,
            Color::new(10.0 / 255.0, 36.0 / 255.0, 106.0 / 255.0, 1.0),
        );
        draw_text(title, dx + 8.0, dy + 18.0, 14.0, COLOR_WHITE);

        let close_rect = Rect::new(dx + dw - 21.0, dy + 5.0, 16.0, 16.0);
        draw_raised_rect(
            close_rect.x,
            close_rect.y,
            close_rect.w,
            close_rect.h,
            1.0,
            Some(COLOR_GRAY),
        );
        draw_line(
            close_rect.x + 4.0,
            close_rect.y + 4.0,
            close_rect.x + 11.0,
            close_rect.y + 11.0,
            2.0,
            COLOR_BLACK,
        );
        draw_line(
            close_rect.x + 4.0,
            close_rect.y + 11.0,
            close_rect.x + 11.0,
            close_rect.y + 4.0,
            2.0,
            COLOR_BLACK,
        );

        (dx, dy, close_rect)
    };

    let draw_btn = |text: &str, rect: Rect, primary: bool| -> bool {
        let is_hover = rect.contains(mouse_pos);
        draw_raised_rect(rect.x, rect.y, rect.w, rect.h, 2.0, Some(COLOR_GRAY));
        if primary {
            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, COLOR_BLACK);
        }
        let dim = measure_text(text, None, 14, 1.0);
        draw_text(
            text,
            rect.x + (rect.w - dim.width) / 2.0,
            rect.y + (rect.h + dim.height) / 2.0 - 1.0,
            14.0,
            COLOR_BLACK,
        );
        is_hover && mouse_pressed
    };

    match dialog {
        DialogState::Custom { cols, rows, mines } => {
            let (dx, dy, close_rect) = draw_window("Juego Personalizado", 300.0, 220.0);
            if close_rect.contains(mouse_pos) && mouse_pressed || is_key_pressed(KeyCode::Escape) {
                *dialog = DialogState::None;
                return DialogEvent::Close;
            }

            let mut y = dy + 45.0;
            draw_text("Columnas (9 - 40):", dx + 16.0, y + 16.0, 14.0, COLOR_BLACK);
            draw_sunken_rect(dx + 160.0, y, 48.0, 22.0, 2.0, Some(COLOR_WHITE));
            draw_text(cols.to_string(), dx + 175.0, y + 16.0, 14.0, COLOR_BLACK);
            if draw_btn("-", Rect::new(dx + 215.0, y, 22.0, 22.0), false) {
                *cols = (*cols).saturating_sub(1).max(9);
            }
            if draw_btn("+", Rect::new(dx + 242.0, y, 22.0, 22.0), false) {
                *cols = (*cols + 1).min(40);
            }

            y += 36.0;
            draw_text("Filas (9 - 24):", dx + 16.0, y + 16.0, 14.0, COLOR_BLACK);
            draw_sunken_rect(dx + 160.0, y, 48.0, 22.0, 2.0, Some(COLOR_WHITE));
            draw_text(rows.to_string(), dx + 175.0, y + 16.0, 14.0, COLOR_BLACK);
            if draw_btn("-", Rect::new(dx + 215.0, y, 22.0, 22.0), false) {
                *rows = (*rows).saturating_sub(1).max(9);
            }
            if draw_btn("+", Rect::new(dx + 242.0, y, 22.0, 22.0), false) {
                *rows = (*rows + 1).min(24);
            }

            y += 36.0;
            let max_m = (*cols * *rows).saturating_sub(1);
            *mines = (*mines).clamp(10, max_m);
            draw_text("Minas:", dx + 16.0, y + 16.0, 14.0, COLOR_BLACK);
            draw_sunken_rect(dx + 160.0, y, 48.0, 22.0, 2.0, Some(COLOR_WHITE));
            draw_text(mines.to_string(), dx + 172.0, y + 16.0, 14.0, COLOR_BLACK);
            if draw_btn("-", Rect::new(dx + 215.0, y, 22.0, 22.0), false) {
                *mines = (*mines).saturating_sub(5).max(10);
            }
            if draw_btn("+", Rect::new(dx + 242.0, y, 22.0, 22.0), false) {
                *mines = (*mines + 5).min(max_m);
            }

            let btn_ok = Rect::new(dx + 45.0, dy + 175.0, 95.0, 26.0);
            let btn_cancel = Rect::new(dx + 160.0, dy + 175.0, 95.0, 26.0);
            if draw_btn("Aceptar", btn_ok, true) || is_key_pressed(KeyCode::Enter) {
                let c = *cols;
                let r = *rows;
                let m = *mines;
                *dialog = DialogState::None;
                return DialogEvent::ApplyCustom(c, r, m);
            }
            if draw_btn("Cancelar", btn_cancel, false) {
                *dialog = DialogState::None;
                return DialogEvent::Close;
            }
        }
        DialogState::HighScores { tab } => {
            let (dx, dy, close_rect) = draw_window("Mejores Tiempos", 360.0, 280.0);
            if close_rect.contains(mouse_pos) && mouse_pressed
                || is_key_pressed(KeyCode::Escape)
                || is_key_pressed(KeyCode::Enter)
            {
                *dialog = DialogState::None;
                return DialogEvent::Close;
            }

            let tabs = [
                ("Principiante", "principiante"),
                ("Intermedio", "intermedio"),
                ("Experto", "experto"),
            ];
            let mut tx = dx + 14.0;
            for (title, key) in tabs {
                let tab_rect = Rect::new(tx, dy + 34.0, 106.0, 24.0);
                let is_active = *tab == key;
                if is_active {
                    draw_raised_rect(
                        tab_rect.x,
                        tab_rect.y,
                        tab_rect.w,
                        tab_rect.h,
                        2.0,
                        Some(COLOR_GRAY),
                    );
                    draw_line(
                        tab_rect.x + 2.0,
                        tab_rect.bottom() - 1.0,
                        tab_rect.right() - 2.0,
                        tab_rect.bottom() - 1.0,
                        2.0,
                        COLOR_GRAY,
                    );
                } else {
                    draw_sunken_rect(
                        tab_rect.x,
                        tab_rect.y,
                        tab_rect.w,
                        tab_rect.h,
                        1.0,
                        Some(COLOR_LIGHT_GRAY),
                    );
                    if tab_rect.contains(mouse_pos) && mouse_pressed {
                        *tab = key;
                    }
                }
                let dim = measure_text(title, None, 13, 1.0);
                draw_text(
                    title,
                    tab_rect.x + (tab_rect.w - dim.width) / 2.0,
                    tab_rect.y + 16.0,
                    13.0,
                    COLOR_BLACK,
                );
                tx += 110.0;
            }

            let panel_x = dx + 12.0;
            let panel_y = dy + 57.0;
            let panel_w = 336.0;
            let panel_h = 160.0;
            draw_sunken_rect(panel_x, panel_y, panel_w, panel_h, 2.0, Some(COLOR_WHITE));

            draw_rectangle(
                panel_x + 2.0,
                panel_y + 2.0,
                panel_w - 4.0,
                22.0,
                Color::new(0.9, 0.9, 0.9, 1.0),
            );
            draw_text("#", panel_x + 10.0, panel_y + 16.0, 13.0, COLOR_BLACK);
            draw_text("Nombre", panel_x + 35.0, panel_y + 16.0, 13.0, COLOR_BLACK);
            draw_text("Tiempo", panel_x + 185.0, panel_y + 16.0, 13.0, COLOR_BLACK);
            draw_text("Fecha", panel_x + 250.0, panel_y + 16.0, 13.0, COLOR_BLACK);

            let scores = highscores.get_scores(tab);
            let mut row_y = panel_y + 38.0;
            for (i, s) in scores.iter().enumerate() {
                let col = if i == 0 {
                    Color::new(0.0, 0.0, 0.6, 1.0)
                } else {
                    COLOR_BLACK
                };
                draw_text((i + 1).to_string(), panel_x + 10.0, row_y, 13.0, col);
                draw_text(&s.name, panel_x + 35.0, row_y, 13.0, col);
                draw_text(format!("{}s", s.time), panel_x + 185.0, row_y, 13.0, col);
                draw_text(&s.date, panel_x + 250.0, row_y, 13.0, COLOR_DARK_GRAY);
                row_y += 24.0;
            }

            if draw_btn(
                "Restablecer",
                Rect::new(dx + 20.0, dy + 235.0, 110.0, 26.0),
                false,
            ) {
                highscores.reset_defaults();
                return DialogEvent::ResetHighScores;
            }
            if draw_btn(
                "Aceptar",
                Rect::new(dx + 240.0, dy + 235.0, 100.0, 26.0),
                true,
            ) {
                *dialog = DialogState::None;
                return DialogEvent::Close;
            }
        }
        DialogState::NewRecord {
            diff_name,
            diff_id,
            seconds,
            name,
        } => {
            let (dx, dy, close_rect) = draw_window("¡Nuevo Récord!", 320.0, 210.0);
            if close_rect.contains(mouse_pos) && mouse_pressed || is_key_pressed(KeyCode::Escape) {
                *dialog = DialogState::None;
                return DialogEvent::Close;
            }

            draw_text(
                format!("¡Felicidades! Ganaste en {}", diff_name),
                dx + 20.0,
                dy + 45.0,
                14.0,
                COLOR_BLACK,
            );
            draw_text(
                format!("Tiempo récord: {} segundos", seconds),
                dx + 20.0,
                dy + 68.0,
                15.0,
                Color::new(0.0, 0.5, 0.0, 1.0),
            );
            draw_text(
                "Introduce tu nombre:",
                dx + 20.0,
                dy + 95.0,
                14.0,
                COLOR_BLACK,
            );

            let box_rect = Rect::new(dx + 20.0, dy + 108.0, 280.0, 28.0);
            draw_sunken_rect(
                box_rect.x,
                box_rect.y,
                box_rect.w,
                box_rect.h,
                2.0,
                Some(COLOR_WHITE),
            );
            draw_text(
                name.as_str(),
                box_rect.x + 8.0,
                box_rect.y + 19.0,
                15.0,
                COLOR_BLACK,
            );

            if ((get_time() * 2.0) as usize).is_multiple_of(2) {
                let dim = measure_text(name.as_str(), None, 15, 1.0);
                let cx = box_rect.x + 8.0 + dim.width + 2.0;
                draw_line(
                    cx,
                    box_rect.y + 5.0,
                    cx,
                    box_rect.y + 23.0,
                    2.0,
                    COLOR_BLACK,
                );
            }

            while let Some(c) = get_char_pressed() {
                if !c.is_control() && name.chars().count() < 16 {
                    name.push(c);
                }
            }
            if is_key_pressed(KeyCode::Backspace) {
                name.pop();
            }

            if draw_btn(
                "Guardar",
                Rect::new(dx + 105.0, dy + 155.0, 110.0, 28.0),
                true,
            ) || is_key_pressed(KeyCode::Enter)
            {
                let saved_name = if name.trim().is_empty() {
                    "Anónimo".to_string()
                } else {
                    name.clone()
                };
                let d_id = diff_id.clone();
                let secs = *seconds;
                *dialog = DialogState::None;
                return DialogEvent::SaveRecord(d_id, saved_name, secs);
            }
        }
        DialogState::Help => {
            let (dx, dy, close_rect) =
                draw_window("Cómo Jugar al Buscaminas v2 (Rust)", 400.0, 310.0);
            if close_rect.contains(mouse_pos) && mouse_pressed
                || is_key_pressed(KeyCode::Escape)
                || is_key_pressed(KeyCode::Enter)
            {
                *dialog = DialogState::None;
                return DialogEvent::Close;
            }

            let instructions = [
                (
                    "Objetivo:",
                    "Descubrir todas las casillas que no contengan minas.",
                ),
                ("Clic Izquierdo:", "Revelar casilla oculta."),
                (
                    "Clic Derecho:",
                    "Poner o quitar bandera / interrogación (?).",
                ),
                ("Chording:", "Clic central o izquierdo+derecho en número:"),
                ("", "si tiene sus banderas, revela las vecinas al instante."),
                (
                    "Primer Clic Seguro:",
                    "¡La primera casilla siempre abre un área limpia!",
                ),
                (
                    "Atajos:",
                    "F2 = Nuevo | 1, 2, 3 = Dificultad | M = Sonido | P = Partículas",
                ),
            ];

            let mut y = dy + 45.0;
            for (title, desc) in instructions {
                if !title.is_empty() {
                    draw_text(title, dx + 20.0, y, 13.0, Color::new(0.0, 0.0, 0.6, 1.0));
                }
                draw_text(desc, dx + 130.0, y, 13.0, COLOR_BLACK);
                y += 28.0;
            }

            if draw_btn(
                "Entendido",
                Rect::new(dx + 145.0, dy + 265.0, 110.0, 26.0),
                true,
            ) {
                *dialog = DialogState::None;
                return DialogEvent::Close;
            }
        }
        DialogState::About => {
            let (dx, dy, close_rect) = draw_window("Acerca de Buscaminas v2", 340.0, 240.0);
            if close_rect.contains(mouse_pos) && mouse_pressed
                || is_key_pressed(KeyCode::Escape)
                || is_key_pressed(KeyCode::Enter)
            {
                *dialog = DialogState::None;
                return DialogEvent::Close;
            }

            let lines = [
                (
                    "Buscaminas v2.4.0 (Edición Rust)",
                    16.0,
                    Color::new(0.0, 0.1, 0.5, 1.0),
                ),
                ("Desarrollado en Rust con Macroquad", 13.0, COLOR_BLACK),
                (
                    "Aceleración nativa por hardware a 60+ FPS,",
                    13.0,
                    COLOR_VERY_DARK,
                ),
                ("audio procedural multiplataforma,", 13.0, COLOR_VERY_DARK),
                (
                    "sistema de partículas y chording auténtico.",
                    13.0,
                    COLOR_VERY_DARK,
                ),
                (
                    "¡Rendimiento ultrarrápido y seguro en memoria!",
                    13.0,
                    Color::new(0.0, 0.45, 0.0, 1.0),
                ),
            ];

            let mut y = dy + 50.0;
            for (text, size, col) in lines {
                let dim = measure_text(text, None, size as u16, 1.0);
                draw_text(text, dx + (340.0 - dim.width) / 2.0, y, size, col);
                y += 24.0;
            }

            if draw_btn(
                "Aceptar",
                Rect::new(dx + 120.0, dy + 195.0, 100.0, 26.0),
                true,
            ) {
                *dialog = DialogState::None;
                return DialogEvent::Close;
            }
        }
        DialogState::None => {}
    }

    DialogEvent::None
}

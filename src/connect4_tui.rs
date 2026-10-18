//! Game TUIs: board rendering + a generic ratatui event loop.
//! `run_tui_for` runs any game described by `TuiDesc`; `GameKind::run`
//! picks the game. Human-vs-human when `ai` is `None`, human-vs-AI when
//! `Some` (AI plays one side via plain closures so this crate never
//! depends on the player crate).

use crate::game::{connect4::Connect4, Playable};
use crossterm::event::{Event, KeyCode, KeyEventKind, read};
use ratatui::{
    DefaultTerminal,
    layout::Constraint::Percentage,
    layout::Layout,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};
use std::fmt::Display;

pub const BOARD_BG: Color = Color::Rgb(0, 0, 139);

/// Think-time choices (seconds) offered on the AI setup screen.
pub const SETUP_SECS: [u64; 4] = [1, 3, 5, 10];

/// The AI side in the TUI: initial seat + display name, a move callback,
/// and a setup callback. Both closures live in the caller (e.g. a CLI
/// binary that depends on both the game and player crates).
/// `initial_sec` preselects the setup screen's think-time row;
/// `apply_setup` runs ONCE with the final (seat, sec) at setup-confirm.
pub struct AiOpponent<'a, T> {
    pub color: &'a str,
    pub name: &'a str,
    pub get_move: &'a dyn Fn(&[String], &str, &T) -> Option<String>,
    pub apply_setup: &'a dyn Fn(&str, u64),
    pub initial_sec: u64,
}

/// Describes one playable game for the generic event loop: window title,
/// selectable positions, board renderer, and "position index -> move".
pub struct TuiDesc<T> {
    pub title: &'static str,
    pub num_positions: usize,
    pub render: fn(&T, usize) -> Vec<Line<'static>>,
    pub move_of: fn(usize) -> String,
}

/// Games the TUI can run. New game = new variant + one `run` arm + one
/// `TuiDesc` (no game-picker UI yet: single game).
pub enum GameKind {
    Connect4,
}

impl GameKind {
    pub const ALL: &'static [GameKind] = &[GameKind::Connect4];

    pub fn title(&self) -> &'static str {
        match self {
            GameKind::Connect4 => "Connect4",
        }
    }

    pub fn run(&self, ai: Option<AiOpponent<Connect4>>) -> anyhow::Result<()> {
        match self {
            GameKind::Connect4 => run_tui_for(
                Connect4::default(),
                TuiDesc {
                    title: self.title(),
                    num_positions: 7,
                    render: board_lines,
                    move_of: |s| (s + 1).to_string(),
                },
                ai,
            ),
        }
    }
}

/// Back-compat wrapper: Connect4 TUI. `None` = human-vs-human.
pub fn run_tui(ai: Option<AiOpponent<Connect4>>) -> anyhow::Result<()> {
    GameKind::Connect4.run(ai)
}

// Board cells via public bit-state API (perspective Red):
// bits 7..49 empty, 49..91 Red ("me"), 91..133 Yellow ("opponent"),
// index 7 + col + row*7. Avoids touching game logic / private fields.
pub fn board_lines(game: &Connect4, selected: usize) -> Vec<Line<'static>> {
    let bits = game.get_bit_state("Red");
    let mut lines = vec![Line::from(format!("Turn: {}", game.get_active_player()))];
    let mut marker: Vec<Span> = Vec::with_capacity(7);
    for i in 0..7 {
        marker.push(if i == selected {
            Span::styled(
                "v ",
                Style::new()
                    .fg(Color::Yellow)
                    .bg(BOARD_BG)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Span::styled("  ", Style::new().bg(BOARD_BG))
        });
    }
    lines.push(Line::from(marker));
    for r in (0..6).rev() {
        let mut row: Vec<Span> = Vec::with_capacity(7);
        for c in 0..7 {
            let i = 7 + c + r * 7;
            row.push(if bits[i] {
                Span::styled(
                    "○ ",
                    Style::new()
                        .fg(Color::White)
                        .bg(BOARD_BG)
                        .add_modifier(Modifier::DIM),
                )
            } else if bits[i + 42] {
                Span::styled("● ", Style::new().fg(Color::LightRed).bg(BOARD_BG))
            } else {
                Span::styled("● ", Style::new().fg(Color::Yellow).bg(BOARD_BG))
            });
        }
        lines.push(Line::from(row));
    }
    let mut nums: Vec<Span> = Vec::with_capacity(7);
    for c in 0..7 {
        let mut style = Style::new().fg(Color::White).bg(BOARD_BG);
        if c == selected {
            style = style.add_modifier(Modifier::REVERSED);
        }
        nums.push(Span::styled(format!("{} ", c + 1), style));
    }
    lines.push(Line::from(nums));
    lines
}

#[derive(PartialEq)]
enum Screen {
    Start,
    Playing,
    GameOver,
}

fn try_play<T: Playable>(game: &mut T, mv: &str, msg: &mut String, screen: &mut Screen) {
    if game.is_terminal() {
        *msg = "Game over, press r to restart.".to_string();
        return;
    }
    if game.get_valid_moves().contains(&mv.to_string()) {
        game.play(mv);
        *msg = format!("Played {mv}.");
        if game.is_terminal() {
            *screen = Screen::GameOver;
        }
    } else {
        *msg = format!("Invalid move {mv}.");
    }
}

fn human_turn<T: Playable>(game: &T, ai_color: Option<&str>) -> bool {
    match ai_color {
        None => true,
        Some(c) => game.get_active_player() != c,
    }
}

/// Run the TUI for one game. `None` = human-vs-human (plain start screen);
/// `Some` = AI setup screen (starter + think-time) then AI plays its seat.
/// AI moves run synchronously with an "AI tenker..." status line drawn first.
pub fn run_tui_for<T: Playable + Display>(
    mut game: T,
    desc: TuiDesc<T>,
    ai: Option<AiOpponent<T>>,
) -> anyhow::Result<()> {
    let mut terminal: DefaultTerminal = ratatui::init();
    let players = game.get_game_static().players; // seat names; [0] starts
    let mut screen = Screen::Start;
    let mut selected: usize = desc.num_positions / 2;
    let mut msg = String::from("Press Enter to start.");
    // AI setup state (only when ai.is_some()): effective seat (setup can
    // flip it), starter + think-time rows.
    let mut ai_color: Option<&str> = ai.as_ref().map(|a| a.color);
    let mut ai_starts = ai
        .as_ref()
        .and_then(|a| players.first().map(|first| first == a.color))
        .unwrap_or(false);
    let mut setup_row: usize = 0; // 0 = starter, 1 = think-time
    let mut sec_idx: usize = ai
        .as_ref()
        .and_then(|a| SETUP_SECS.iter().position(|s| *s == a.initial_sec))
        .unwrap_or(1); // 3 sek
    loop {
        let setup = screen == Screen::Start && ai.is_some();
        let ai_thinking = screen == Screen::Playing
            && !game.is_terminal()
            && !human_turn(&game, ai_color);
        terminal.draw(|f| {
            let [left, right] =
                Layout::horizontal([Percentage(60), Percentage(40)]).areas(f.area());
            match screen {
                Screen::Start if setup => {
                    let ai_name = ai.as_ref().map(|a| a.name).unwrap_or("AI");
                    f.render_widget(
                        Paragraph::new(format!(
                            "{} vs {}\n\n{}Starter: {}\n{}Tenketid: {} sek\n\nEnter=start q/Esc=quit",
                            desc.title,
                            ai_name,
                            if setup_row == 0 { "> " } else { "  " },
                            if ai_starts { "AI" } else { "Du" },
                            if setup_row == 1 { "> " } else { "  " },
                            SETUP_SECS[sec_idx],
                        ))
                        .block(Block::bordered().title(desc.title)),
                        left,
                    )
                }
                Screen::Start => f.render_widget(
                    Paragraph::new(format!(
                        "{0}\n\nPress Enter to start.\nq/Esc to quit.",
                        desc.title
                    ))
                    .block(Block::bordered().title(desc.title)),
                    left,
                ),
                _ => f.render_widget(
                    Paragraph::new((desc.render)(&game, selected))
                        .block(Block::bordered().title(desc.title)),
                    left,
                ),
            };
            let mut right_text = match screen {
                Screen::Start if setup => {
                    "Up/Down=velg rad\nLeft/Right=endre\nEnter=start\nq/Esc=avslutt".to_string()
                }
                Screen::Start => {
                    format!("Welcome to {}.\n\nEnter=start\nq/Esc=quit", desc.title)
                }
                Screen::Playing => format!(
                    "Player: {}\nValid: {}\nMsg: {msg}\n\n1-7/Left-Right+Enter=play\nr=reset q/Esc=quit",
                    game.get_active_player(),
                    game.get_valid_moves().join(","),
                ),
                Screen::GameOver => format!(
                    "Winner: {}\nMsg: {msg}\n\nr=restart q=quit",
                    game.get_winner().unwrap_or("Draw".to_string()),
                ),
            };
            if ai_thinking {
                right_text.push_str("\nAI tenker...");
            }
            if screen != Screen::Start && game.is_terminal() {
                right_text.push_str(&format!(
                    "\nTerminal! Winner: {}",
                    game.get_winner().unwrap_or("Draw".to_string())
                ));
            }
            f.render_widget(
                Paragraph::new(right_text).block(Block::bordered().title("Status/Dialog")),
                right,
            );
        })?;
        if ai_thinking {
            let ai_op = ai.as_ref().expect("ai_thinking implies Some");
            let moves = game.get_valid_moves();
            let player = game.get_active_player();
            match (ai_op.get_move)(&moves, &player, &game) {
                Some(mv) => try_play(&mut game, &mv, &mut msg, &mut screen),
                None => msg = "AI found no move.".to_string(),
            }
            continue;
        }
        let Event::Key(key) = read()? else { continue };
        if key.kind != KeyEventKind::Press {
            continue;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => break,
            KeyCode::Enter => match screen {
                Screen::Start => match ai.as_ref() {
                    Some(ai_op) => {
                        let seat = if ai_starts {
                            players.first().map(|s| s.as_str()).unwrap_or(ai_op.color)
                        } else {
                            players.get(1).map(|s| s.as_str()).unwrap_or(ai_op.color)
                        };
                        let sec = SETUP_SECS[sec_idx];
                        (ai_op.apply_setup)(seat, sec);
                        ai_color = Some(seat);
                        screen = Screen::Playing;
                        msg = "Good luck!".to_string();
                    }
                    None => {
                        screen = Screen::Playing;
                        msg = "Good luck!".to_string();
                    }
                },
                Screen::Playing => {
                    if human_turn(&game, ai_color) {
                        try_play(&mut game, &(desc.move_of)(selected), &mut msg, &mut screen)
                    }
                }
                Screen::GameOver => {}
            },
            KeyCode::Char('r') => {
                game.reset();
                selected = desc.num_positions / 2;
                screen = Screen::Playing;
                msg = "Restarted.".to_string();
            }
            KeyCode::Up => {
                if setup {
                    setup_row = setup_row.saturating_sub(1);
                }
            }
            KeyCode::Down => {
                if setup {
                    setup_row = (setup_row + 1).min(1);
                }
            }
            KeyCode::Left => {
                if setup {
                    if setup_row == 0 {
                        ai_starts = false;
                    } else {
                        sec_idx = (sec_idx + SETUP_SECS.len() - 1) % SETUP_SECS.len();
                    }
                } else {
                    selected = selected.saturating_sub(1);
                }
            }
            KeyCode::Right => {
                if setup {
                    if setup_row == 0 {
                        ai_starts = true;
                    } else {
                        sec_idx = (sec_idx + 1) % SETUP_SECS.len();
                    }
                } else {
                    selected = (selected + 1).min(desc.num_positions.saturating_sub(1));
                }
            }
            KeyCode::Char(c @ '1'..='9') => {
                if screen == Screen::Playing && human_turn(&game, ai_color) {
                    let idx = (c as usize) - ('1' as usize);
                    if idx < desc.num_positions {
                        selected = idx;
                        try_play(&mut game, &(desc.move_of)(selected), &mut msg, &mut screen);
                    }
                }
            }
            _ => {}
        }
    }
    ratatui::restore();
    Ok(())
}

use boardgame_game::connect4_tui::board_lines;
use boardgame_game::game::{connect4::Connect4, Playable};
use crossterm::event::{Event, KeyCode, KeyEventKind, read};
use ratatui::{
    DefaultTerminal,
    layout::Constraint::Percentage,
    layout::Layout,
    widgets::{Block, Paragraph},
};

#[derive(PartialEq)]
enum Screen {
    Start,
    Playing,
    GameOver,
}

fn try_play(game: &mut Connect4, mv: &str, msg: &mut String, screen: &mut Screen) {    if game.is_terminal() {
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

fn main() -> anyhow::Result<()> {
    let mut terminal: DefaultTerminal = ratatui::init();
    let mut game = Connect4::default();
    let mut screen = Screen::Start;
    let mut selected: usize = 3; // 0-based, col 4
    let mut msg = String::from("Press Enter to start.");
    loop {
        terminal.draw(|f| {
            let [left, right] =
                Layout::horizontal([Percentage(60), Percentage(40)]).areas(f.area());
            // cursor marker line, cheap column highlight
            match screen {
                Screen::Start => f.render_widget(
                    Paragraph::new("Connect4\n\nPress Enter to start.\nq/Esc to quit.")
                        .block(Block::bordered().title("Connect4")),
                    left,
                ),
                _ => f.render_widget(
                    Paragraph::new(board_lines(&game, selected))
                        .block(Block::bordered().title("Connect4")),
                    left,
                ),
            };
            let mut right_text = match screen {
                Screen::Start => "Welcome to Connect4.\n\nEnter=start\nq/Esc=quit".to_string(),
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
        let Event::Key(key) = read()? else { continue };
        if key.kind != KeyEventKind::Press {
            continue;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => break,
            KeyCode::Enter => match screen {
                Screen::Start => {
                    screen = Screen::Playing;
                    msg = "Good luck!".to_string();
                }
                Screen::Playing => try_play(&mut game, &(selected + 1).to_string(), &mut msg, &mut screen),
                Screen::GameOver => {}
            },
            KeyCode::Char('r') => {
                game.reset();
                selected = 3;
                screen = Screen::Playing;
                msg = "Restarted.".to_string();
            }
            KeyCode::Left => selected = selected.saturating_sub(1),
            KeyCode::Right => selected = (selected + 1).min(6),
            KeyCode::Char(c @ '1'..='7') => {
                if screen == Screen::Playing {
                    selected = (c as usize) - ('1' as usize);
                    try_play(&mut game, &c.to_string(), &mut msg, &mut screen);
                }
            }
            _ => {}
        }
    }
    ratatui::restore();
    Ok(())
}

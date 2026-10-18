//! Connect4 board rendering shared by the game TUI binaries.
//! Rendering only: no event loop, no AI, no Agentish.

use crate::game::{connect4::Connect4, Playable};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

pub const BOARD_BG: Color = Color::Rgb(0, 0, 139);

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

use core::str;
pub mod connect4;
pub mod nim12;

use log::trace;

#[derive(Clone)]
pub struct GameStatic {
    pub name: String,
    pub players: Vec<String>,
    state_size: usize, // number of bits
}

impl GameStatic {
    pub fn new(name: String, players: Vec<String>, state_size: usize) -> GameStatic {
        Self {
            name,
            players,
            state_size,
        }
    }

    pub fn get_state_size(&self) -> usize {
        self.state_size
    }
}

pub trait Playable {
    fn reset(&mut self);
    fn get_game_static(&self) -> GameStatic;
    fn get_valid_moves(&self) -> Vec<String>;
    fn is_terminal(&self) -> bool;
    fn get_winner(&self) -> Option<String>;
    fn get_active_player(&self) -> String;
    fn play(&mut self, play_move: &str);
    fn get_bit_state(&self, perspective: &str) -> Vec<bool>;
    fn get_bit_state_from_bit_state_and_move(
        &self,
        perspective: &str,
        tmp_state: &[bool],
        cmove: &str,
    ) -> Vec<bool>;
    fn get_valid_moves_from_bit_state(
        &self,
        perspective: &str,
        tmp_state: &[bool],
    ) -> Vec<String>;
    fn pretty_print(&self);
}

pub enum TerminalState {
    Me,
    Opponent,
    Draw,
}

impl std::fmt::Debug for TerminalState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TerminalState::Me => write!(f, "Me"),
            TerminalState::Opponent => write!(f, "Oponent"),
            TerminalState::Draw => write!(f, "Draw"),
        }
    }
}

// Bit-position encoding for game state vectors.
// Layout (from `connect4::Connect4::get_game_static` test, and `get_active_player_from_bit_state`):
//   [0]   constant false
//   [1]   constant true
//   [2,3] perspective bits: xor determines active player
//   [4]   winner_is_none (set when no terminal)
//   [5]   winner_is_perspective
//   [6]   winner_is_opponent
//   [7..] board (7 cols × 6 rows = 42 cells per layer, 2 layers = 84 bits)
// Total state size for Connect4 is 132 (1+1+2+3+125 ≈ 132 with padding).
pub const BIT_CONST_FALSE: usize = 0;
pub const BIT_CONST_TRUE: usize = 1;
pub const BIT_PERSPECTIVE_A: usize = 2;
pub const BIT_PERSPECTIVE_B: usize = 3;
pub const BIT_TERMINAL_DRAW: usize = 4;
pub const BIT_TERMINAL_ME: usize = 5;
pub const BIT_TERMINAL_OPP: usize = 6;
pub const BIT_BOARD_START: usize = 7;

pub fn get_terminal_state_from_bit_state(state: &Vec<bool>) -> Option<TerminalState> {
    if state[BIT_TERMINAL_DRAW] {
        trace!("get_terminal_state_from_bit_state {:?}", state);
        return Some(TerminalState::Draw);
    } else if state[BIT_TERMINAL_ME] {
        return Some(TerminalState::Me);
    } else if state[BIT_TERMINAL_OPP] {
        trace!("get_terminal_state_from_bit_state {:?}", state);
        return Some(TerminalState::Opponent);
    }
    None
}

pub fn get_active_player_from_bit_state(game_static: &GameStatic, state: &Vec<bool>) -> String {
    if state[BIT_PERSPECTIVE_A] ^ state[BIT_PERSPECTIVE_B] {
        game_static.players[0].clone()
    } else {
        game_static.players[1].clone()
    }
}

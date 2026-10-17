use crate::game::*;

    use crate::game::Playable;
    use super::GameStatic;
    use log::log_enabled;
    use log::Level::Debug;
    use log::{debug, trace};
    //use std::collections::HashMap;
    use std::fmt::{self};

    #[derive(Debug, Clone)]
    pub struct Nim12 {
        turn: bool,
        tokens_left: u8,
    }

    impl Playable for Nim12 {
        fn reset(&mut self) {
            self.turn = false;
            self.tokens_left = 20;

        }

        fn get_valid_moves(&self) -> Vec<String> {
            if self.tokens_left == 0 {
                vec![]
            } else if self.tokens_left == 1 {
                return vec!["1".to_string()];
            } else {
                return vec!["1".to_string(), "2".to_string()];
            }
        }

        fn is_terminal(&self) -> bool {
            let moves = &self.get_valid_moves();
            moves.is_empty()
        }
        fn get_winner(&self) -> Option<String> {
            if self.is_terminal() {
                if !self.turn {
                    Some("B".to_string())
                } else {
                    Some("A".to_string())
                }
            } else {
                panic!("Game is not terminalted!");
            }
        }
        fn get_game_static(&self) -> GameStatic {
            GameStatic::new(
                "1-2 Nim".to_string(),
                vec!["A".to_string(), "B".to_string()],
                8, //not counting is_active_player_bit and always false and always true
                /*HashMap::from([
                    ("is_active_player".to_string(), 0),
                    ("always false".to_string(), 1),
                    ("always true".to_string(), 2),
                    ("turn".to_string(), 3),
                    ("bit_1".to_string(), 4),
                    ("bit_2".to_string(), 5),
                    ("bit_4".to_string(), 6),
                    ("bit_8".to_string(), 7),
                    ("bit_16".to_string(), 8),
                    ("bit_32".to_string(), 9),
                    ("bit_64".to_string(), 10),
                ]),
                vec!["1".to_string(), "2".to_string()],
                initial_state,*/
            )
        }

        fn get_active_player(&self) -> String {
            if !self.turn {
                "A".to_string()
            } else {
                "B".to_string()
            }
        }

        fn play(&mut self, play_move: &str) {
            // Check if right player is playing
            //if player_name.to_string().ne(self.get_active_player()) {
            //    panic!("Wrong PLAYER you are {} expected {}", player_name, self.get_active_player());
            //}
            let played: u8 = match play_move {
                "1" => 1,
                "2" => 2,
                _ => {
                    panic!("Unknown move");
                }
            };
            self.tokens_left -= played;
            self.turn = !self.turn;
            debug!(
                "end of move: game state:{} => {} {}",
                play_move, self.turn, self.tokens_left
            );
        }

        fn get_bit_state(&self, _perspective: &str) -> Vec<bool> {
            let value = self.tokens_left;
            let mut number_as_bool = Nim12::u8_to_vec_bool(value);
            number_as_bool[0] = self.turn;
            number_as_bool
        }

        fn get_bit_state_from_bit_state_and_move(
            &self,
            _perspective: &str,
            tmp_state: &[bool],
            play_move: &str,
        ) -> Vec<bool> {
            let (mut turn, mut tokens_left) = get_state_from_bit_state(tmp_state);
            let played: u8 = match play_move {
                "1" => 1,
                "2" => 2,
                _ => {
                    panic!("Unknown move");
                }
            };

            tokens_left -= played;
            turn = !turn;

            get_bit_state_from_state(turn, tokens_left)
        }
        fn get_valid_moves_from_bit_state(
            &self,
            _perspective: &str,
            _tmp_state: &[bool],
        ) -> Vec<String> {
            todo!("Unimplemented");
        }

        fn pretty_print(&self) {
            todo!("Unimplemented. See fmt");
        }
    }

    // Help functions
    fn get_bit_state_from_state(turn: bool, tokens_left: u8) -> Vec<bool> {
        let value = tokens_left;
        let mut number_as_bool = Nim12::u8_to_vec_bool(value);
        number_as_bool[0] = turn;
        number_as_bool
    }
    fn get_state_from_bit_state(bit_state: &[bool]) -> (bool, u8) {
        //0 -> me or other
        //1 -> false
        //2 -> true
        //3 -> false=A,true=B
        //4-10 -> tokens_left
        let turn = bit_state[0];
        let mut bit_tokens = bit_state.to_owned();
        bit_tokens.reverse();

        if log_enabled!(target: "Global", Debug) {
            let mut bits = "".to_string();
            for bit in &bit_tokens {
                bits.push_str(format!("{}-", bit).as_str());
            }
            trace!("{}", bits);
        }
        let mut tokens_left = Nim12::vec_bool_to_u8(&bit_tokens); //bit∏_tokens.iter().fold(0u8, |v, b| (v << 1) + (*b as u8));
        if tokens_left >= 128 {
            tokens_left -= 128;
        }
        (turn, tokens_left)
    }

    impl fmt::Display for Nim12 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Turn: {} - tokens left: {}",
                match self.turn {
                    false => "A",
                    true => "B",
                },
                self.tokens_left
            )
        }
    }

    impl Nim12 {
        /*   fn u8_to_array_bool(value: u8) -> [bool; 8] {
            let mut arr = [false; 8];
            for i in 0..8 {
                arr[7 - i] = (value & (1 << i)) != 0;
            }
            arr
        }
        */
        fn u8_to_vec_bool(value: u8) -> Vec<bool> {
            (0..8).rev().map(|i| (value & (1 << i)) != 0).collect()
        }

        fn vec_bool_to_u8(vecv: &[bool]) -> u8 {
            let mut value = 0;
            for (i, b) in vecv.iter().enumerate() {
                if *b {
                    value |= 1 << i;
                }
            }
            value
        }
    }

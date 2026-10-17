    use crate::game::GameStatic;
    use crate::game::*;
    use log::{debug, info, trace};
    use std::fmt::{self};
    use inline_colorization::*;

    #[derive(Debug, Clone, PartialEq)]
    enum C4Player {
        Red,
        Yellow,
    }
    impl fmt::Display for C4Player {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    C4Player::Red => "Red".to_string(),
                    C4Player::Yellow => "Yellow".to_string(),
                },
            )
        }
    }
    #[derive(Debug)]
    pub struct Connect4 {
        turn: C4Player,
        table: [[u8; 6]; 7], // 1 for red, 2 for yellow,
        // 0 for empty
        is_terminated: bool,
        winner: Option<C4Player>,
        last_printed_table: [[u8; 6]; 7],
    }

    impl Default for Connect4 {
        fn default() -> Self {

            Connect4 {
                turn: C4Player::Red,
                table: [[0; 6]; 7],
                is_terminated: false,
                winner: None,
                last_printed_table: [[0; 6]; 7],

            }
        }
    }
    impl crate::game::Playable for Connect4 {
        fn reset(&mut self) {
            self.turn = C4Player::Red;
            self.table = [[0; 6]; 7];
            self.is_terminated = false;
            self.winner = None;

        }

        fn get_valid_moves(&self) -> Vec<String> {
            let mut valid_moves: Vec<String> = vec![];
            for column in 0..7 {
                if self.table[column][5] == 0 {
                    valid_moves.push((column + 1).to_string());
                }
            }
            valid_moves
        }

        fn is_terminal(&self) -> bool {
            self.is_terminated
        }
        fn get_winner(&self) -> Option<String> {
            if self.is_terminated {
                let win = self.winner.clone();
                win.map(|val| val.to_string())
            } else {
                None
            }
        }
        fn get_game_static(&self) -> GameStatic {
            GameStatic::new(
                "Connect4".to_string(),
                vec!["Red".to_string(), "Yellow".to_string()],
                132, //not counting is_active_player_bit and always false and always true
            )
        }

        fn get_active_player(&self) -> String {
            match self.turn {
                C4Player::Red => "Red".to_string(),
                C4Player::Yellow => "Yellow".to_string(),
            }
        }

        fn play(&mut self, play_move: &str) {
            // Check if right player is playing
            //if player_name.to_string().ne(self.get_active_player()) {
            //    panic!("Wrong PLAYER you are {} expected {}", player_name, self.get_active_player());
            //}

            // save state before move
            for r in (0..6).rev() {
                for c in 0..7 {
                    self.last_printed_table[c][r] = self.table[c][r];
                }
            }

            let played: usize = match play_move {
                "1" => 0,
                "2" => 1,
                "3" => 2,
                "4" => 3,
                "5" => 4,
                "6" => 5,
                "7" => 6,
                x => {
                    panic!("Unknown move: {x}");
                }
            };
            // warn!("connect4: play startXXX {}", self.turn);
            //todo!("Plasser brikke");
            let mut row: usize = 0;

            while row < 6 {
                if self.table[played][row] == 0 {
                    self.table[played][row] = match self.get_active_player().as_str() {
                        "Red" => 1,
                        "Yellow" => 2,
                        _ => panic!("Unknown player"),
                    };
                    break;
                } else {
                    row += 1;
                    // warn!("{row}");
                }
            }
            let current_pice = self.table[played][row];
            let mut in_row: i8 = 1;

            // check below
            if row > 2 {
                let mut xrow: i8 = row as i8;
                while xrow > 0 {
                    xrow -= 1;
                    if current_pice == self.table[played][xrow as usize] {
                        in_row += 1;
                        continue;
                    }
                    break;
                }
                // check for win
                if in_row > 3 {
                    self.is_terminated = true;
                    self.winner = match self.get_active_player().as_str() {
                        "Red" => Some(C4Player::Red),
                        "Yellow" => Some(C4Player::Yellow),
                        _ => panic!("Unknown player"),
                    };
                    debug!("{} won by column", self.get_active_player());
                    return ;
                }
            }

            //check sides
            in_row = 1;
            //check left
            let mut xcol: i8 = played as i8;
            while xcol > 0 {
                xcol -= 1;
                if current_pice == self.table[xcol as usize][row] {
                    in_row += 1;
                    continue;
                }
                break;
            }
            // warn!("connect4: play A XXX");

            //check right
            xcol = played as i8;
            while xcol < 6 {
                xcol += 1;
                if current_pice == self.table[xcol as usize][row] {
                    in_row += 1;
                    continue;
                }
                break;
            }

            // check for win
            if in_row > 3 {
                self.is_terminated = true;
                self.winner = match self.get_active_player().as_str() {
                    "Red" => Some(C4Player::Red),
                    "Yellow" => Some(C4Player::Yellow),
                    _ => panic!("Unknown player"),
                };
                debug!("{} won by row", self.get_active_player());
                return ;
            }

            //check lowerleft to upper right
            in_row = 1;
            xcol = played as i8;
            let mut xrow: i8 = row as i8;
            while xcol > 0 && xrow > 0 {
                xcol -= 1;
                xrow -= 1;
                if current_pice == self.table[xcol as usize][xrow as usize] {
                    in_row += 1;
                    continue;
                }
                break;
            }

            xcol = played as i8;
            xrow = row as i8;
            while xcol < 6 && xrow < 5 {
                xcol += 1;
                xrow += 1;
                if current_pice == self.table[xcol as usize][xrow as usize] {
                    in_row += 1;
                    continue;
                }
                break;
            }

            // check for win
            if in_row > 3 {
                self.is_terminated = true;
                self.winner = match self.get_active_player().as_str() {
                    "Red" => Some(C4Player::Red),
                    "Yellow" => Some(C4Player::Yellow),
                    _ => panic!("Unknown player"),
                };
                trace!("{} won by lowerleft to upper right", self.get_active_player());
                return ;
            }

            //check lowerright to upper left
            in_row = 1;
            xcol = played as i8;
            xrow = row as i8;
            while xcol > 0 && xrow < 5 {
                xcol -= 1;
                xrow += 1;
                if current_pice == self.table[xcol as usize][xrow as usize] {
                    in_row += 1;
                    continue;
                }
                break;
            }

            xcol = played as i8;
            xrow = row as i8;
            while xcol < 6 && xrow > 0 {
                xcol += 1;
                xrow -= 1;
                if current_pice == self.table[xcol as usize][xrow as usize] {
                    in_row += 1;
                    continue;
                }
                break;
            }

            // check for win
            if in_row > 3 {
                self.is_terminated = true;
                self.winner = match self.get_active_player().as_str() {
                    "Red" => Some(C4Player::Red),
                    "Yellow" => Some(C4Player::Yellow),
                    _ => panic!("Unknown player"),
                };
                debug!("{} won by upper left to lower right", self.get_active_player());
            }
            //warn!("connect4: play end win XXX");
            if self.turn == C4Player::Yellow && self.table[0][5] > 0
                    && self.table[1][5] > 0
                    && self.table[2][5] > 0
                    && self.table[3][5] > 0
                    && self.table[4][5] > 0
                    && self.table[5][5] > 0 && self.table[6][5] > 0 {
                self.is_terminated = true;
                self.winner = None;
                return ;
            }
            // warn!("connect4: play before toggle XXX {}", self.turn);

            //toggle player turn
            self.turn = match self.turn {
                C4Player::Red => C4Player::Yellow,
                C4Player::Yellow => C4Player::Red,
            };
            //   warn!("connect4: play after toggle XXX {}", self.turn);
            //debug!("end of move: game state:{} => {:?}", play_move, self);

        }

        /// perspective is a &str with either value Red or Yellow
        ///
        /// Bit_state is made for the AI and should be made with the AI in mind.
        /// What kind of bit is important for the AI?
        fn get_bit_state(&self, perspective: &str) -> Vec<bool> {
            // bits
            // 0 -> false
            // 1 -> true
            // 2 -> sin tur: true=perspective,false=oponenet
            // 3 -> false perspective is red, true perspective is yellow,  (which order is the perspective?)

            // 4 -> winner_is_none
            // 5 -> winner_is_perspective(first person)
            // 6 -> winner_is_oponent
            // 7-48   -> slot_is_empty[7][6]
            // 49-90  -> slot_perspective[7][6]
            // 91-132 -> slot_oponent[7][6]
            // = 133 bits

            //        turn: C4Player,
            // table: [[u8; 7]; 6], // 0= empty 1= Red,2=Yellow
            // is_terminated: bool,
            // winner: Option<C4Player>,
            // let value = self.tokens_left;
            //let mut number_as_bool = Connect4::u8_to_vec_bool(value);
            // number_as_bool[0] = self.turn;
            //number_as_bool
            //vec![false];

            // 0 -> false
            // 1 -> true
            let mut retur = vec![false, true];
            // 2 -> me(true) or oponent(false) (the player with the next move ?)
            // 2 -> me or oponent (the player with the next move ?) same as turn but in bool form
            let me_color = match perspective {
                "Red" => C4Player::Red,
                "Yellow" => C4Player::Yellow,
                x => panic!("UNKNOWN COLOR {x}"),
            };
            if me_color == self.turn {
                retur.push(true); // perspective 2
            } else {
                retur.push(false); // oponent 2
            }

            // 3 ->0 red, 1 yellow (who am I(perspective)?)
            retur.push(match me_color {
                C4Player::Red => false,
                C4Player::Yellow => true,
            });

            // 4 -> winner_is_none
            // 5 -> winner_is_red
            // 6 -> winner_is_yellow
            if self.is_terminal() {
                match self.get_winner() {
                    Some(x) => {
                        /*              match x.as_str() {
                            "Red" => retur.extend([false, true, false]),
                            "Yellow" => retur.extend([false, false, true]),
                            "None" => retur.extend([true, false, false]),
                            x => panic!("Unkown player {}", x),
                        };*/
                        if x.as_str() == perspective {
                            retur.extend([false, true, false]);
                        } else if x.as_str() == "None" {
                            //                          panic!("DEBUG DRAW{x} != {perspective}");
                            retur.extend([true, false, false]);
                        } else {
                            //                           panic!("DEBUG {x} != {perspective}");
                            retur.extend([false, false, true]);
                        }
                    }

                    None => retur.extend([false, false, false]),
                }
            } else {
                retur.extend([false, false, false]);
            }

            //7-132
            retur.extend([false; 42 * 3]);

            for column in 0..7 {
                for row in 0..6 {
                    let bit_address = 7 + column + row * 7; // table starts at bit 7

                    match self.table[column][row] {
                        0u8 => retur[bit_address] = true,
                        1 => match perspective {
                            //1= Red pice|
                            "Red" => retur[bit_address + 42] = true, // mark as me
                            "Yellow" => retur[bit_address + 42 * 2] = true, // mark as opponent
                            x => panic!("UNKNOWN COLOR {x}"),
                        },

                        2 => match perspective {
                            //2 = Yellow pice
                            "Red" => retur[bit_address + 42 * 2] = true, // mark as opponent
                            "Yellow" => retur[bit_address + 42] = true,  // mark as me
                            x => panic!("UNKNOWN COLOR {x}"),
                        },
                        i => panic!("Unknown number {i}"),
                    }
                }
            }
            //debug!("get_bit_state: {:?}", retur);
            retur
        }

        /// Returns the state of the game as a bit vector.
        /// The state of the game is represented as a bit vector.
        /// Used by Agentish.get_move()
        fn get_bit_state_from_bit_state_and_move(
            &self,
            perspective: &str,
            tmp_state: &[bool],
            play_move: &str,
        ) -> Vec<bool> {
            let mut next_self = new_from_bit_state(perspective, tmp_state);
            let possible_moves = next_self.get_valid_moves();
            let play_moves_string = play_move.to_string();
            if !possible_moves.contains(&play_moves_string) {
                panic!("{} is not a valid move", play_moves_string);
            }
            next_self.play(play_move);
            if false {
                info!("next {:?}", next_self);
                /*
                if (next_self.is_terminated) {
                    warn!("next2: {:?}", next_self);
                    panic!("Game AI see the END. (This is something I want)");
                }*/
            }
            next_self.get_bit_state(perspective)
        }

        fn get_valid_moves_from_bit_state(
            &self,
            perspective: &str,
            tmp_state: &[bool],
        ) -> Vec<String> {
            let curr_self = new_from_bit_state(perspective, tmp_state);

            curr_self.get_valid_moves()
        }

        fn pretty_print(&self) {
            println!(
                "Turn: {}",
                match self.turn {
                    C4Player::Red => "Red",
                    C4Player::Yellow => "Yellow",
                },
            );
            for r in (0..6).rev() {
                for c in 0..7 {
                    match self.table[c][r] == self.last_printed_table[c][r] {
                        true =>
                            print!("{color_white}{} ",
                            match self.table[c][r] {
                                1 => "X",
                                2 => "O",
                                0 => " ",
                                _ => "E",})
                            ,
                        false => print!("{color_yellow}{} ",
                            match self.table[c][r] {
                                1 => "X",
                                2 => "O",
                                0 => " ",
                                _ => "E",
                        }),
                    }

                }
                println!();
            }
            println!("1 2 3 4 5 6 7 \n");

            // save print state


        }


    }

    // Help functions

    /// perspective is whom is interrested of highest score?
    pub fn new_from_bit_state(perspective: &str, bit_state: &[bool]) -> Connect4 {
        // bits
        // 0 -> false
        // 1 -> true
        // 2 -> sin tur: true=perspective,false=oponenet
        // 3 -> false perspective is red, true perspective is yellow (Which player is the perspective?)
        // 4 -> winner_is_none
        // 5 -> winner_is_perspective(first person)
        // 6 -> winner_is_oponent
        // 7-48   -> slot_is_empty[7][6]
        // 49-90  -> slot_perspective[7][6]
        // 91-132 -> slot_oponent[7][6]
        // = 133 bits

        //        turn: C4Player,
        // table: [[u8; 7]; 6], // 0= empty 1= Red,2=Yellow
        // is_terminated: bool,
        // winner: Option<C4Player>,
        // let value = self.tokens_left;
        //let mut number_as_bool = Connect4::u8_to_vec_bool(value);
        // number_as_bool[0] = self.turn;
        //number_as_bool
        //vec![false];

        let turn = !bit_state[3] ^ bit_state[2]; //0 is Red, //1 is Yellow (next to do move)

        let mut table: [[u8; 6]; 7] = [[0; 6]; 7]; // row = 0 => bunn, 6=>topp
        for (column, rows) in table.iter_mut().enumerate() {

            // clippy complain about this line, it result in cluccy code.
            for row in 0..rows.len() {
                let bit_address = 7 + column + row * 7;
                if bit_state[bit_address + 42] {
                    rows[row] = match perspective {
                        "Red" => 1,
                        "Yellow" => 2,
                        x => panic!("Unknown perspective: {}", x),
                    }
                } else if bit_state[bit_address + (42 * 2)] {
                    rows[row] = match perspective {
                        "Red" => 2,
                        "Yellow" => 1,
                        x => panic!("Unknown perspective: {}", x),
                    }
                }
            }
        }
        let is_terminated = bit_state[4] | bit_state[5] | bit_state[6];
        let winner: Option<C4Player>;
        if bit_state[5] {
            winner = match bit_state[3] {
                true => Some(C4Player::Red),
                false => Some(C4Player::Yellow),
            };
        } else if bit_state[6] {
            winner = match bit_state[3] {
                true => Some(C4Player::Yellow),
                false => Some(C4Player::Red),
            };
        } else {
            winner = None;
        };
        Connect4 {
            turn: if turn {
                C4Player::Yellow //Yellow = true
            } else {
                C4Player::Red //Red = false
            },
            table,
            is_terminated,
            winner,
            last_printed_table: table,
        }
    }

    impl fmt::Display for Connect4 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(
                f,
                "Turn: {}",
                match self.turn {
                    C4Player::Red => "Red",
                    C4Player::Yellow => "Yellow",
                },
            )?;
            for r in (0..6).rev() {
                for c in 0..7 {
                    write!(
                        f,
                        "{} ",
                        // choose symbol
                        match self.table[c][r] {
                            1 => "X",
                            2 => "O",
                            0 => " ",
                            _ => "E",
                        }
                    )?;
                }
                writeln!(f)?;
            }
            writeln!(f, "1 2 3 4 5 6 7 ")?;

            Ok(())
        }
    }


#[cfg(test)]
mod tests {
    use super::*;
    //use Connect4;
    use crate::game::Playable;

    #[test]
    fn initial_state_perspective_red() {
        let game = Connect4::default();
        assert_eq!(game.get_game_static().name, "Connect4");
        let result = game.get_bit_state("Red");
        assert_eq!(result.len(), 133);

        assert_eq!(result[0], false, "0 should be false"); // 0 -> false
        assert_eq!(result[1], true, "1 should be true"); // 1 -> true
        assert_eq!(result[2], true, "2 should be true"); // 2 -> sin tur: true=perspective,false=oponenet
        assert_eq!(result[3], false, "[3] should be false"); // 3 -> false perspective is red, true perspective is yellow (Which player is the perspective?)
        assert_eq!(result[4], false, "4 should be false"); // 4 -> winner_is_none
        assert_eq!(result[5], false, "5 should be false"); // 5 -> winner_is_perspective(first person)
        assert_eq!(result[6], false, "6 should be false"); // // 6 -> winner_is_oponent
        assert_eq!(result[7], true, "7: is (0,0) = empty"); // // 6 -> winner_is_oponent
    }

    #[test]
    fn initial_state_perspective_yellow() {
        let game = Connect4::default();
        assert_eq!(game.get_game_static().name, "Connect4");
        let result = game.get_bit_state("Yellow");
        assert_eq!(result.len(), 133);

        assert_eq!(result[0], false, "0 should be false"); // 0 -> false
        assert_eq!(result[1], true, "1 should be true"); // 1 -> true
        assert_eq!(result[2], false, "2 should be false"); // 2 -> sin tur: true=perspective,false=oponenet
        assert_eq!(result[3], true, "3 should be true"); // 3 -> false perspective is red, true perspective is yellow (Which player is the perspective?)
        assert_eq!(result[4], false, "0 should be false"); // 4 -> winner_is_none
        assert_eq!(result[5], false, "0 should be false"); // 5 -> winner_is_perspective(first person)
        assert_eq!(result[6], false, "0 should be false"); // // 6 -> winner_is_oponent
    }

    #[test]
    fn first_move_red() {
        let mut game = Connect4::default();
        assert_eq!(game.get_game_static().name, "Connect4");
        let init_bit_state = game.get_bit_state("Red");
        assert_eq!(init_bit_state[2], true, "2:->true. Red first player");
        assert_eq!(init_bit_state[3], false, "3:->false. Perspective is red");
        let result = game.get_bit_state_from_bit_state_and_move("Red", &init_bit_state, "1");
        assert_eq!(result.len(), 133);

        assert_eq!(result[0], false, "0 should be false"); // 0 -> false
        assert_eq!(result[1], true, "1 should be true"); // 1 -> true
        assert_eq!(result[2], false, "2->false: Yellow second player"); // 2 -> sin tur: true=perspective,false=oponenet
        assert_eq!(result[3], false, "[3] should be false"); // 3 -> false perspective is red, true perspective is yellow (Which player is the perspective?)
        assert_eq!(result[4], false, "0 should be false"); // 4 -> winner_is_none
        assert_eq!(result[5], false, "0 should be false"); // 5 -> winner_is_perspective(first person)
        assert_eq!(result[6], false, "0 should be false"); // // 6 -> winner_is_oponent
        assert_eq!(result[7], false, "7: is (0,0) = empty");
        assert_eq!(result[7 + 42], true, "7+42: is (0,0) = mine");
        assert_eq!(result[7 + 42 * 2], false, "7+42*2: is (0,0) = oponent");

        game.play("1");
        let result2 = game.get_bit_state("Red");
        assert_eq!(result, result2, "state_bit is equal after move");

        let result3 = game.get_bit_state("Yellow");
        assert_eq!(
            result3[7 + 42],
            false,
            "7+42: (0,0) is not = mine (after move)"
        );
        assert_eq!(result3[7 + 42 * 2], true, "7+42*2: is (0,0) = oponent");
    }

    #[test]
    fn terminal_game() {
        use super::*;
        use crate::game::connect4::connect4::new_from_bit_state;
        let mut game = Connect4::default();
        game.play("1");
        game.play("2");
        game.play("1");
        game.play("2");
        game.play("1");
        game.play("2");

        let init_bit_state = game.get_bit_state("Red");
        let mut game2 = new_from_bit_state("Red", &init_bit_state);
        assert_eq!(
            game2.get_active_player(),
            "Red".to_string(),
            "It is reds turn."
        );
        game2.play("1");
        assert_eq!(game2.is_terminal(), true, "Game2 ended");
        assert_eq!(
            game2.get_bit_state("Red")[5],
            true,
            "Game2 ended and red won."
        );

        let end_state = game.get_bit_state_from_bit_state_and_move("Red", &init_bit_state, "1");
        //game.play("1"); ok.
        // Red won
        //let end_state = game.get_bit_state("Red");
        assert_eq!(
            end_state[2], true,
            "2->true: Red turn since game ended on Red turn."
        );
        assert_eq!(end_state[4], false, "4->false: No draw");
        assert_eq!(end_state[6], false, "6->false: Yellow loose");
        assert_eq!(end_state[5], true, "5->true: Red should win");
    }
}

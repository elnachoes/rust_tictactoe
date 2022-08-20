use crate::objects::{GameOverState, PlayerTurn, SpaceState};

#[derive(Clone, Debug)]
pub struct GameState {
    pub board_state : Vec<Vec<SpaceState>>,
    pub player_turn : PlayerTurn
}

impl GameState {
    pub fn new() -> GameState {
        GameState { board_state: vec![vec![SpaceState::new(); 3]; 3], player_turn: PlayerTurn::new() }
    }

    pub fn switch_turns(&mut self) {
        self.player_turn.switch();
    }

    pub fn claim_space(&mut self, select_string : &String) -> bool {
        if select_string.len() == 2 {
            let x_index : usize;
            let y_index : usize;
            let y_selection = select_string.chars().nth(0).unwrap();
            let x_selection = select_string.chars().nth(1).unwrap();

            match x_selection {
                '0' => { x_index = 0 }
                '1' => { x_index = 1 }
                '2' => { x_index = 2 }
                _ => { return false }
            }

            match y_selection {
                '0' => { y_index = 0 }
                '1' => { y_index = 1 }
                '2' => { y_index = 2 }
                _ => { return false }
            }

            if self.board_state[x_index][y_index] != SpaceState::Empty {
                return false
            }

            match self.player_turn {
                PlayerTurn::Player1 => { self.board_state[x_index][y_index].claim(&self.player_turn) }
                PlayerTurn::Player2 => { self.board_state[x_index][y_index].claim(&self.player_turn) }
            }

            return true
        }
        else {
            return false
        }
    }

    fn check_if_tie(&self) -> bool {
        for i in &self.board_state {
            for j in i {
                match j {
                    SpaceState::Empty => { return false }
                    _ => {}
                }
            } 
        }
        true
    }

    //TODO : implement
    pub fn determine_game_over_state(&self) -> GameOverState {
        //horizontal/vertical win
        for x in 0..3 {
            if self.board_state[0][x] == self.board_state[1][x] && self.board_state[1][x] == self.board_state[2][x] {
                if self.board_state[0][x] == SpaceState::Player1 {
                    return GameOverState::Player1Win
                } 
                else if self.board_state[0][x] == SpaceState::Player2 {
                    return GameOverState::Player2Win
                }
            }

            if self.board_state[x][0] == self.board_state[x][1] && self.board_state[x][1] == self.board_state[x][2] {
                if self.board_state[x][0] == SpaceState::Player1 {
                    return GameOverState::Player1Win
                } 
                else if self.board_state[x][0] == SpaceState::Player2 {
                    return GameOverState::Player2Win
                }
            }
        }
        //diagonal win
        if self.board_state[0][0] == self.board_state[1][1] && self.board_state[1][1] == self.board_state[2][2] {
            if self.board_state[0][0] == SpaceState::Player1 {
                return GameOverState::Player1Win
            } 
            else if self.board_state[0][0] == SpaceState::Player2 {
                return GameOverState::Player2Win
            }
        }

        if self.board_state[0][2] == self.board_state[1][1] && self.board_state[1][1] == self.board_state[2][0] {
            if self.board_state[0][2] == SpaceState::Player1 {
                return GameOverState::Player1Win
            } 
            else if self.board_state[0][2] == SpaceState::Player2 {
                return GameOverState::Player2Win
            }
        }

        if self.check_if_tie() {
            return GameOverState::Tie
        }
        GameOverState::NotOver
    }
}
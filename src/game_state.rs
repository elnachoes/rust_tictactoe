#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpaceState {
    Empty,
    Player1,
    Player2,
}

#[derive(Clone, Debug)]
pub enum GameOverState {
    NotOver,
    Player1Win,
    Player2Win,
    Tie
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub board_state : Vec<Vec<SpaceState>>,
    pub player_1_turn : bool,
}

impl GameState {
    pub fn new() -> GameState {
        GameState { board_state: vec![vec![SpaceState::Empty; 3]; 3], player_1_turn: true }
    }

    pub fn switch_turns(&mut self) {
        self.player_1_turn = !self.player_1_turn;
    }

    pub fn claim(&mut self, select_string : &String) -> bool {
        if select_string.len() == 2 {
            let mut x_index = 0;
            let mut y_index = 0;
            let x_selection = select_string.chars().nth(0).unwrap();
            let y_selection = select_string.chars().nth(1).unwrap();

            match x_selection {
                '0' => { x_index = 0; }
                '1' => { x_index = 1; }
                '2' => { x_index = 2; }
                _ => { return false; }
            }
            match y_selection {
                '0' => { y_index = 0; }
                '1' => { y_index = 1; }
                '2' => { y_index = 2; }
                _ => { return false; }
            }

            if self.player_1_turn {
                self.board_state[x_index][y_index] = SpaceState::Player1;
            }
            else {
                self.board_state[x_index][y_index] = SpaceState::Player2;
            }

            return true;
        }
        else {
            return false;
        }
    }

    //TODO : implement
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
        let mut player_won = false;
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
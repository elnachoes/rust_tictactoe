use crate::objects::{GameOverState, PlayerTurn, SpaceState};

// this is the main gamestate struct. it stores the state of the gameboard, it determines the state of the gameover, and it determines the turn of the player,
// and it handles claiming spaces
#[derive(Clone, Debug)]
pub struct GameState { pub board_state : Vec<Vec<SpaceState>>, pub player_turn : PlayerTurn }

impl GameState {
    // when a gamestate is constructed it should be completely empty
    pub fn new() -> GameState { GameState { board_state: vec![vec![SpaceState::new(); 3]; 3], player_turn: PlayerTurn::new() } }

    // this switches the player turns
    pub fn switch_turns(&mut self) { self.player_turn.switch(); }

    // this fn will claim a space but if the space is taken or doesn't exist it will return false
    // this fn will read in a string of 2 characters like this for example "00" to define the x and y of where to place a marker
    pub fn claim_space(&mut self, select_string : &String) -> bool {
        if select_string.len() == 2 {

            // these will determine the the x and y index on the board and if they are invalid return false
            let x_index = match select_string.chars().nth(1).unwrap() {
                '0' => { 0 }
                '1' => { 1 }
                '2' => { 2 }
                _ => { return false }
            };

            let y_index = match select_string.chars().nth(0).unwrap() {
                '0' => { 0 }
                '1' => { 1 }
                '2' => { 2 }
                _ => { return false }
            };

            // if the space is already taken return false
            if self.board_state[x_index][y_index] != SpaceState::Empty { return false }

            // this updates the board spaces depending on the turn and the index of where to put the marker
            match self.player_turn {
                PlayerTurn::Player1 => { self.board_state[x_index][y_index].claim(&self.player_turn) }
                PlayerTurn::Player2 => { self.board_state[x_index][y_index].claim(&self.player_turn) }
            }

            return true
        }
        return false
    }

    // this fn determines what the gameover state is weather it is a win for player 1 or 2, a tie, or it is still ongoing
    pub fn determine_game_over(&self) -> GameOverState {
        for x in 0..3 {
            //this is checking for a vertical win or a horizontal win
            if self.board_state[0][x] == self.board_state[1][x] && self.board_state[1][x] == self.board_state[2][x] && self.board_state[2][x] != SpaceState::Empty ||
                self.board_state[x][0] == self.board_state[x][1] && self.board_state[x][1] == self.board_state[x][2] && self.board_state[x][2] != SpaceState::Empty {
                return GameOverState::determine_winner(&self.player_turn)
            }
        }
        //this is checking for a diagonal win
        if self.board_state[0][0] == self.board_state[1][1] && self.board_state[1][1] == self.board_state[2][2] && self.board_state[2][2] != SpaceState::Empty || 
            self.board_state[2][0] == self.board_state[1][1] && self.board_state[1][1] == self.board_state[0][2] && self.board_state[0][2] != SpaceState::Empty {
            return GameOverState::determine_winner(&self.player_turn)
        }
        //this determines if there is a tie or if the game is ongoing because there wasn't a win or a tie
        for i in &self.board_state {
            for j in i {
                match j {
                    SpaceState::Empty => { return GameOverState::NotOver }
                    _ => {}
                }
            }
        }
        //if there aren't anymore empty spaces and there hasn't been a win it is a tie
        GameOverState::Tie
    }
}
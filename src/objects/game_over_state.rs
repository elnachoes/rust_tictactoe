use crate::{PlayerTurn};

// this enum is used to track what the current gameover state is
#[derive(Clone, Debug)]
pub enum GameOverState { NotOver, Player1Win, Player2Win, Tie }

impl GameOverState {
    pub fn determine_winner(player_turn : &PlayerTurn) -> GameOverState {
        match player_turn {
            PlayerTurn::Player1 => { GameOverState::Player1Win }
            PlayerTurn::Player2 => { GameOverState::Player2Win }
        }
    }
}
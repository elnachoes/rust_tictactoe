use crate::objects::PlayerTurn;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpaceState {
    Empty,
    Player1,
    Player2,
}

impl SpaceState {
    pub fn new() -> SpaceState {
        Self::Empty
    }

    pub fn claim(&mut self, current_player_turn : PlayerTurn) {
        *self = match current_player_turn {
            PlayerTurn::Player2 => { Self::Player1 }
            PlayerTurn::Player1 => { Self::Player2 }
        }
    }
}
// this is just a enum to store the state of the player turn
#[derive(Clone, Debug)]
pub enum PlayerTurn { Player1, Player2 }

impl PlayerTurn {
    pub fn new() -> PlayerTurn { Self::Player1 }

    // this will switch the turn of the enum if it is player 1 switch to 2 and vise versa
    pub fn switch(&mut self) {
        *self = match *self {
            Self::Player1 => { Self::Player2 }
            Self::Player2 => { Self::Player1 }
        }
    }
}
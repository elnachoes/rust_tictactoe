#[derive(Clone, Debug)]
pub enum PlayerTurn {
    Player1,
    Player2
}

impl PlayerTurn {
    pub fn new() -> PlayerTurn {
        Self::Player1
    }

    pub fn switch(&mut self) {
        *self = match *self {
            Self::Player1 => { Self::Player2 }
            Self::Player2 => { Self::Player1 }
        }
    }
}
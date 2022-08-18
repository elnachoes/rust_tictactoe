#[derive(Clone, Debug)]
pub enum GameOverState {
    NotOver,
    Player1Win,
    Player2Win,
    Tie
}
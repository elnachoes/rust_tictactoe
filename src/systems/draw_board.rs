use crate::{GameState, SpaceState};

pub fn draw_board(game_state : &GameState) {
    println!("  0  1  2");
    let mut itter = 0;
    for i in &game_state.board_state {
        let mut row_string = String::from(itter.to_string());
        for j in i {
            match j {
                SpaceState::Empty => { row_string.push_str("[ ]"); }
                SpaceState::Player1 => { row_string.push_str("[x]"); }
                SpaceState::Player2 => { row_string.push_str("[o]"); }
            }
        }
        println!("{}",row_string);
        itter += 1;
    }
}
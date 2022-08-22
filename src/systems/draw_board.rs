use crate::{
    GameState, 
    SpaceState
};

// this will draw the board for 
pub fn draw_board(game_state : &GameState) {
    //print the top of the board
    println!("    0  1  2 (x)");
    let mut iter = 0;
    for i in &game_state.board_state {
        let mut row_string = String::from(" ");
        row_string.push_str(iter.to_string().as_mut_str());
        row_string.push_str(" ");
        for j in i {
            match j {
                SpaceState::Empty => { row_string.push_str("[ ]") }
                SpaceState::Player1 => { row_string.push_str("[X]") }
                SpaceState::Player2 => { row_string.push_str("[O]") }
            }
        }
        println!("{}",row_string);
        iter += 1
    }
    println!("(y)")
}
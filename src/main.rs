use tictactoe::{GameState, SpaceState, GameOverState};
use std::io;

fn draw_board(game_state : &GameState) {
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

fn clean_input(input : &mut String) {
    *input = input.replace("\r", "").replace("\n", "");
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();

    // main program loop
    loop {
        let mut game_state = GameState::new();

        // game loop
        loop {
            draw_board(&game_state);
            if game_state.player_1_turn {
                println!("Player 1 Turn (xy): ")
            }
            else {
                println!("Player 2 Turn (xy): ")
            }
            
            input.clear();
            stdin.read_line(&mut input);
            clean_input(&mut input);
    
            let is_valid_input = game_state.claim(&input);
            if is_valid_input {
                game_state.switch_turns();
                match game_state.determine_game_over_state() {
                    GameOverState::Player1Win => { 
                        draw_board(&game_state); 
                        println!("PLAYER 1 WINS!"); 
                        break; 
                    }
                    GameOverState::Player2Win => { 
                        draw_board(&game_state); 
                        println!("PLAYER 2 WINS!"); 
                        break; 
                    }
                    GameOverState::Tie => { 
                        draw_board(&game_state); 
                        println!("TIE!"); 
                        break; 
                    } 
                    GameOverState::NotOver => {}
                }
            }
            else {
                println!("invalid input try again (xy)");
                continue;
            }
        }

        // play again loop
        loop {
            println!("play again y/n ?");
            input.clear();
            stdin.read_line(&mut input);
    
            match input.to_lowercase().chars().nth(0).unwrap() {
                'y' => { break }
                'n' => { return }
                _ => { println!("invalid input, y/n"); continue}
            }
    
        }
    }
}
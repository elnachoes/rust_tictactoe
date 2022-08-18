use crate::{GameState, PlayerTurn, GameOverState};
use crate::{draw_board, clean_input};
use std::io;


/// this is the main game loop for tic tac toe
pub fn game_loop() {
    let mut input = String::new();
    let stdin = io::stdin();

    // main program loop
    loop {
        let mut game_state = GameState::new();

        // game loop
        loop {
            draw_board(&game_state);

            match game_state.player_turn {
                PlayerTurn::Player1 => { println!("Player 1 Turn (xy): ") }
                PlayerTurn::Player2 => { println!("Player 2 Turn (xy): ") }
            }
            
            input.clear();
            stdin.read_line(&mut input);
            clean_input(&mut input);
    
            let is_valid_input = game_state.claim_space(&input);
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
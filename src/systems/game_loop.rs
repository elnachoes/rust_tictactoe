use crate::{ GameState, PlayerTurn, GameOverState, get_input, draw_board };

/// this is the main game loop for tic tac toe
pub fn game_loop() {

    // main game loop
    loop {
        let mut game_state = GameState::new();

        // game loop
        loop {
            draw_board(&game_state);

            match game_state.player_turn {
                PlayerTurn::Player1 => { println!("Player 1 Turn (xy): ") }
                PlayerTurn::Player2 => { println!("Player 2 Turn (xy): ") }
            }

            let input = get_input();
            let is_valid_input = game_state.claim_space(&input);
            if is_valid_input {
                match game_state.determine_game_over() {
                    GameOverState::Player1Win => {
                        draw_board(&game_state);
                        println!("PLAYER 1 WINS!");
                        break
                    }
                    GameOverState::Player2Win => {
                        draw_board(&game_state);
                        println!("PLAYER 2 WINS!");
                        break
                    }
                    GameOverState::Tie => {
                        draw_board(&game_state);
                        println!("TIE!");
                        break
                    }
                    GameOverState::NotOver => {}
                }
                game_state.switch_turns()
            }
            else {
                println!("invalid input try again (xy)");
                continue
            }
        }

        // play again loop
        loop {
            println!("play again y/n ?");
            let input = get_input();
            match input.to_lowercase().as_str() {
                "y" | "yes" => { break }
                "n" | "no" => { return }
                _ => { println!("invalid input, y/n"); continue }
            }
        }
    }
}
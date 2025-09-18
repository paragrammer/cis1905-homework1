use std::io::BufReader;
use theseus::*;

fn wait() {
    std::thread::sleep(std::time::Duration::from_millis(300));
}

fn show_with_message(game: &Game, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    game.show();
    println!("{}", message);
    Ok(())
}

fn game_over(game: &Game) -> Result<bool, Box<dyn std::error::Error>> {
    match game.status() {
        GameStatus::Win => {
            show_with_message(&game, "You win!")?;
            Ok(true)
        }
        GameStatus::Lose => {
            show_with_message(&game, "You lose!")?;
            Ok(true)
        }
        GameStatus::Continue => Ok(false),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read command line arguments
    if std::env::args().len() != 2 {
        println!("Usage: theseus <board_path>");
        std::process::exit(1);
    }
    let board_path = std::env::args().nth(1).unwrap();
    let board = std::fs::read_to_string(board_path).expect("Cannot find board file");

    // Initialize game struct
    let mut game = Game::from_board(&board)?;

    // Game loop
    loop {
        show_with_message(&game, "")?;

        // read user input
        let cmd = loop {
            match input(BufReader::new(std::io::stdin())) {
                Some(cmd) => break cmd,
                None => {
                    show_with_message(&game, "Invalid command. Please try again.")?;
                }
            }
        };

        game.theseus_move(cmd);

        if game_over(&game)? {
            break;
        }

        show_with_message(&game, "Minotaurs turn 1...")?;
        wait();

        game.minotaur_move();

        if game_over(&game)? {
            break;
        }

        show_with_message(&game, "Minotaurs turn 2...")?;
        wait();

        game.minotaur_move();

        if game_over(&game)? {
            break;
        }
    }

    Ok(())
}

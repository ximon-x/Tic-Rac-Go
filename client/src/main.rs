use crate::tui::entry::terminal;
use crate::tui::game::{Game, Player};

use std::io::{self, Error, ErrorKind};

pub mod tui;

use figlet_rs::FIGfont;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Tic Rac Go!");

    let mut game: Game;
    let mut dimension = String::new();
    let mut player = String::new();

    clearscreen::clear().expect("failed to clear screen");
    println!("{}", figure.unwrap());

    println!("Enter Dimension (n x n), defaults to 3. n? ");
    match stdin.read_line(&mut dimension) {
        Ok(_) => {
            match dimension.trim().parse::<u32>() {
                Ok(i) => {
                    if i > 9 {
                        println!("Grid too large, defaulting to 3");
                        game = Game::new(None, 3);
                    } else {
                        game = Game::new(None, i as usize);
                    }
                }
                Err(..) => {
                    println!("Invalid input, defaulting to 3");
                    game = Game::new(None, 3);
                }
            };
        }
        Err(error) => {
            print!("Error: {}, defaulting to 3", error);
            game = Game::new(None, 3);
        }
    };

    println!("Play as X or O? ");
    match stdin.read_line(&mut player) {
        Ok(_) => match (&player as &str).trim() {
            "X" => game.player = Player::X,
            "O" => game.player = Player::O,
            _ => {
                println!("Invalid input, defaulting to X");
                game.player = Player::X;
            }
        },
        Err(error) => {
            print!("Error: {}, defaulting to X", error);
            game.player = Player::X;
        }
    };

    // Checking server health
    let url = "http://localhost:3000/";
    let resp = reqwest::blocking::get(url);

    match resp {
        Ok(_) => (),
        Err(_) => {
            return Err(Box::new(Error::new(
                ErrorKind::ConnectionRefused,
                format!("Server is not running at {}", url),
            )));
        }
    }

    terminal(&mut game)?;
    Ok(())
}

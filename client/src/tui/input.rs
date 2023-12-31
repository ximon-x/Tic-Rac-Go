use std::io::{self, Stdout};

use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use serde_json::Value;
use tui_input::{backend::crossterm::EventHandler, Input};

use super::game::{Game, Player};

pub fn handle_input(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    input: &mut Input,
    game: &mut Game,
) -> Result<u8, io::Error> {
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Char('q') => {
                // restore terminal
                disable_raw_mode()?;
                execute!(
                    terminal.backend_mut(),
                    LeaveAlternateScreen,
                    DisableMouseCapture
                )?;
                terminal.show_cursor()?;

                return Ok(1);
            }

            KeyCode::Enter => {
                // Validate input
                if input.value().len() != 2 {
                    input.reset();
                    return Ok(2);
                }

                let l = game.board.len() - 1;

                let file_char = input.value().chars().next().unwrap();
                let rank_char = input.value().chars().last().unwrap();

                if !file_char.is_alphabetic() || !rank_char.is_digit(10) {
                    input.reset();
                    return Ok(2);
                }

                let r = (file_char as u8 - 'A' as u8) as usize;
                let c = (rank_char as u8 - '1' as u8) as usize;

                if r > l || c > l {
                    input.reset();
                    return Ok(2);
                }

                if game.board[r][c] != 0 {
                    input.reset();
                    return Ok(2);
                }

                if game.player == Player::X {
                    game.board[r][c] = 1;
                } else {
                    game.board[r][c] = 2;
                }

                let serialized = serde_json::to_string(&game).unwrap();

                let url = "http://localhost:3000/play";
                let client = reqwest::blocking::Client::new();

                let res = client
                    .post(url)
                    .body(serialized)
                    .header("Content-Type", "application/json")
                    .send()
                    .unwrap();

                let res_str = res.text().unwrap();

                let val: Value =
                    serde_json::from_str(&res_str.as_str()).unwrap();

                match val["move"].clone() {
                    Value::Array(moves) => {
                        let r = moves[0].as_u64().unwrap() as usize;
                        let c = moves[1].as_u64().unwrap() as usize;

                        if game.player == Player::X {
                            game.board[r][c] = 2;
                        } else {
                            game.board[r][c] = 1;
                        }
                    }
                    _ => {}
                }

                game.game_over = val["game_over"].as_bool().unwrap();

                input.reset();
            }

            KeyCode::Esc => {
                input.reset();
            }

            _ => {
                input.handle_event(&Event::Key(key));
            }
        }
    }

    Ok(0)
}

use std::io;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Terminal,
};
use serde_json::Value;
use tui_input::{backend::crossterm::EventHandler, Input};

use crate::{Game, Player};

pub fn terminal(game: &mut Game) -> Result<(), io::Error> {
    let mut input = Input::default();

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    loop {
        if game.game_over {
            // restore terminal
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            terminal.show_cursor()?;
            println!("Game Over!");
            break;
        }

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(5),
                        Constraint::Percentage(10),
                        Constraint::Percentage(70),
                        Constraint::Percentage(15),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let header = Block::default()
                .title(String::from("Tic Rac Go!"))
                .borders(Borders::NONE)
                .title_alignment(Alignment::Center)
                .style(Style::default().fg(Color::Yellow));

            let input = Paragraph::new(input.value())
                .style(Style::default())
                .block(Block::default().borders(Borders::ALL).title("Input"));

            let table_width = vec![Constraint::Length(5); game.board.len()];

            let body = Table::new(game.board.iter().map(|row| {
                Row::new(row.iter().map(|cell| {
                    match cell {
                        0 => Cell::from(Text::from("-"))
                            .style(Style::default().fg(Color::Green)),

                        1 => Cell::from(Text::from("X"))
                            .style(Style::default().fg(Color::Red)),

                        2 => Cell::from(Text::from("O"))
                            .style(Style::default().fg(Color::Blue)),

                        _ => panic!("Invalid cell value"),
                    }
                }))
                .height(2)
            }))
            .style(Style::default().fg(Color::White))
            .header(
                Row::new(vec![Cell::from(Text::from("Board"))])
                    .style(Style::default().fg(Color::Yellow))
                    .bottom_margin(1),
            )
            .widths(&table_width)
            .column_spacing(1);

            let footer = Paragraph::new(Text::from(
                r"Press 'q' to quit, 'esc' to clear input, and 'enter' to submit your input eg 'A2'.",
            ))
            .style(Style::default().fg(Color::Green))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Powered by Ratatui and Crossterm").border_style(
                        Style::default()
                            .fg(Color::Yellow)
                    )
            );

            f.render_widget(header, chunks[0]);
            f.render_widget(input, chunks[1]);
            f.render_widget(body, chunks[2]);
            f.render_widget(footer, chunks[3]);
        })?;

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

                    break;
                }

                KeyCode::Enter => {
                    // Validate input
                    if input.value().is_empty() {
                        continue;
                    }

                    if input.value().len() > 2 {
                        input.reset();
                        continue;
                    }

                    let c = (input.value().chars().nth(0).unwrap() as u8 - b'A')
                        as usize;
                    let r = (input.value().chars().nth(1).unwrap() as i32
                        - 0x30
                        - 1) as usize;

                    let l = game.board.len() - 1;

                    if game.board[r][c] != 0 || r > l || c > l {
                        input.reset();
                        continue;
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
    }

    Ok(())
}

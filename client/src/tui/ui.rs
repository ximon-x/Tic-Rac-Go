use std::io::{self, Stdout};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Terminal,
};
use tui_input::Input;

use super::game::Game;

pub fn draw(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    input: &Input,
    game: &Game,
) -> Result<(), io::Error> {
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

    Ok(())
}

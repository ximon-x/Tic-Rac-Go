use std::io;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui_input::Input;

use crate::Game;

use super::{input::handle_input, ui::draw};

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

        draw(&mut terminal, &input, &game).unwrap();

        match handle_input(&mut terminal, &mut input, game).unwrap() {
            1 => break,
            2 => continue,
            _ => (),
        }
    }

    Ok(())
}

// src/app.rs

use crate::society::Society;
use crate::game::Game;
use crate::ui::{draw_dashboard, draw_group_page, Page};

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io::{self, stdout};
use std::time::Duration;


pub fn run_app(game: &mut Game) -> Result<(), io::Error> {
    let mut stdout = stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut current_page = Page::Dashboard;
    let mut selected_index = 0;

    loop {
        terminal.draw(|f| {
            match current_page {
                Page::Dashboard => {
                    draw_dashboard(f, &game, selected_index);
                }
                Page::GroupDetail(index) => {
                    draw_group_page(f, &game.society.interest_groups[index]);
                }
            }
        })?;

        // Handle input (Vim-style)
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match current_page {
                    Page::Dashboard => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('j') => {
                            if selected_index + 1 < game.society.interest_groups.len() {
                                selected_index += 1;
                            }
                        }
                        KeyCode::Char('k') => {
                            if selected_index > 0 {
                                selected_index -= 1;
                            }
                        }
                        KeyCode::Char('l') => {
                            current_page = Page::GroupDetail(selected_index);
                        }
                        KeyCode::Enter => {
                            game.next_turn(); // Advance on enter
                        }
                        _ => {}
                    },
                    Page::GroupDetail(_) => match key.code {
                        KeyCode::Char('h') => {
                            current_page = Page::Dashboard;
                        }
                        KeyCode::Char('q') => break,
                        _ => {}
                    },
                }
            }
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}


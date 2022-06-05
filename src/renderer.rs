// use chrono::prelude::*;
use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
// use rand::{distributions::Alphanumeric, prelude::*};
use serde::{Deserialize, Serialize, de::Expected};
use std::fs;
use std::io;
use std::sync::mpsc;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};

use crate::Event;
use crate::channels;

enum MenuItem {
    Home,
    Channels,
    Messages,
    Input,
    Search,
}

pub fn render_windows(rx: &mpsc::Receiver<Event<crossterm::event::KeyEvent>>) -> Result<(), Box<dyn std::error::Error>>{
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let window_titles = vec![
        "Home",
        "Channels",
        "Messages",
        "Input",
        "Search",
    ];

    let mut active_window_item = MenuItem::Home;
    let mut channel_list_state = ListState::default();
    channel_list_state.select(Some(0));

    loop {
        // Windows layout
        terminal.draw(|rect| {
            let size = rect.size();
            let root_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(20),   // Channels
                        Constraint::Min(2),      // Messages
                    ]
                    .as_ref(),
                )
                .split(size);
            
            // Render channels
            let channels: Vec<_> = window_titles
                .iter()
                .map(|title| {
                    ListItem::new(Spans::from(vec![Span::styled(
                        title.clone(),
                        Style::default(),
                    )]))
                }).collect();

            rect.render_stateful_widget(channels::render_channels(&channel_list_state), root_chunks[0], &mut channel_list_state);
        })?;

        // TODO: Move to function
        // Receive event from input thread
        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char('h') => active_window_item = MenuItem::Home,
                KeyCode::Char('c') => active_window_item = MenuItem::Channels,
                KeyCode::Char('a') => {
                    
                }
                KeyCode::Char('d') => {
                    
                }
                KeyCode::Down => {

                }
                KeyCode::Up => {

                }
                _ => {}
            },
            Event::Tick => {},
            Event::Quit | Event::Change(_) => todo!(),
        }
    }

    Ok(())
}
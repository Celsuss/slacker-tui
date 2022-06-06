// use chrono::prelude::*;
use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent, KeyModifiers},
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

use crate::{Event};
use crate::channels;
use crate::home;
use crate::messages;

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

    let mut active_window_item = MenuItem::Channels;
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

            rect.render_stateful_widget(channels::render_channels(&channel_list_state, matches!(active_window_item, MenuItem::Channels)), root_chunks[0], &mut channel_list_state);

            // Render home
            // rect.render_widget(home::render_home(), root_chunks[1]);

            let messages_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [Constraint::Percentage(90),
                            Constraint::Percentage(10)].as_ref(),
                        )
                        .split(root_chunks[1]);

            // Render messages
            rect.render_widget(messages::render_messages(), messages_chunks[0]);
            rect.render_widget(messages::render_messages_input(matches!(active_window_item, MenuItem::Input)), messages_chunks[1]);
        })?;

        // TODO: Move to function
        // Receive event from input thread
        match rx.recv()? {
            Event::Input(event) => match event {
                KeyEvent{ code: KeyCode::Char('q'), modifiers: KeyModifiers::NONE} => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }

                KeyEvent{ code: KeyCode::Up, modifiers: KeyModifiers::NONE} => {

                }
                KeyEvent{ code: KeyCode::Right, modifiers: KeyModifiers::NONE} => {
                    active_window_item = MenuItem::Input;
                }
                KeyEvent{ code: KeyCode::Down, modifiers: KeyModifiers::NONE} => {

                }
                KeyEvent{ code: KeyCode::Left, modifiers: KeyModifiers::NONE} => {
                    active_window_item = MenuItem::Channels;
                }
                _ => {}
            },
            Event::Tick => {},
            Event::Quit | Event::Change(_) => todo!(),
        }
    }

    Ok(())
}
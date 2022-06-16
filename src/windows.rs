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
use crate::input_reciever;
use crate::slack_interface::{user_interface, channel_interface};


#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MenuItem {
    None,
    Home,
    Channels,
    Teams,
    Users,
    Messages,
    Input,
    Search,
}

// Convert MenuItem to usize, will be used to
// highlight the current menu item using Tabs in TUI component
impl From<MenuItem> for usize {
    fn from(item: MenuItem) -> usize {
        match item {
            MenuItem::None => 0,
            MenuItem::Messages => 1,
            MenuItem::Input => 2,
            MenuItem::Users => 3,
            MenuItem::Channels => 4,
            MenuItem::Teams => 5,
            MenuItem::Search => 6,
            MenuItem::Home => 7,
        }
    }
}

pub fn render_windows(rx: &mpsc::Receiver<Event<crossterm::event::KeyEvent>>) -> Result<(), Box<dyn std::error::Error>>{
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let config = crate::parse_config().expect("Parse config expect");
    let oauth_token = config["oauth_token"].as_str().expect("OAuth token is not a string");
    let user_list = user_interface::get_user_list(oauth_token).expect("Get user list expect");
    let channel_list = channel_interface::get_channel_list(oauth_token).expect("Get channel list expect");

    let window_titles = vec![
        "Home",
        "Channels",
        "Teams",
        "Users",
        "Messages",
        "Input",
        "Search",
    ];

    let mut active_window_item = MenuItem::Channels;
    let mut focus_window_item = MenuItem::None;

    let mut channel_list_state = ListState::default();
    let mut team_list_state = ListState::default();
    let mut user_list_state = ListState::default();
    channel_list_state.select(Some(0));
    team_list_state.select(Some(0));
    user_list_state.select(Some(0));

    loop {
        // Windows layout
        terminal.draw(|rect| {
            let size = rect.size();
            let root_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(20),   // Channels
                        Constraint::Min(2),      // Messages
                    ]
                    .as_ref(),
                )
                .split(size);
            
            // Render teams, channels and users
            let channels_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(10),     // Teams
                        Constraint::Percentage(55),     // Channels
                        Constraint::Percentage(35),     // Users
                    ]
                    .as_ref(),
                )
                .split(root_chunks[0]);

            rect.render_stateful_widget(
                channels::render_teams(&team_list_state,
                    matches!(active_window_item, MenuItem::Teams), 
                    matches!(focus_window_item, MenuItem::Teams)),
                channels_chunks[0],
                &mut team_list_state);
            rect.render_stateful_widget(
                channels::render_channels(&channel_list, 
                    &channel_list_state, 
                    matches!(active_window_item, MenuItem::Channels), 
                    matches!(focus_window_item, MenuItem::Channels)),
                channels_chunks[1],
                &mut channel_list_state);
            rect.render_stateful_widget(
                channels::render_users(&user_list, 
                    &user_list_state, 
                    matches!(active_window_item, MenuItem::Users), 
                    matches!(focus_window_item, MenuItem::Users)),
                channels_chunks[2],
                &mut user_list_state);


            // Render messages and messages input
            let messages_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [Constraint::Percentage(90),
                            Constraint::Percentage(10)].as_ref(),
                        )
                        .split(root_chunks[1]);

            rect.render_widget(messages::render_messages(), messages_chunks[0]);
            rect.render_widget(messages::render_messages_input(matches!(active_window_item, MenuItem::Input)), messages_chunks[1]);
        })?;

        // TODO: Handle exit event
        let event = input_reciever::recieve_input(rx,
            &mut active_window_item, 
            &mut focus_window_item,
            &mut channel_list_state,
            &mut user_list_state).expect("Input expect");
        if matches!(event, Event::Quit){
            disable_raw_mode()?;
            terminal.show_cursor()?;
            break;
        }
    }

    Ok(())
}
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

use crate::{InputEvent, messages::Conversation};
use crate::channels;
use crate::home;
use crate::messages;
use crate::input_reciever::{InputReciever};
use crate::slack_interface::{user_interface, channel_interface, messages_interface};


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

pub struct ConversationList<T>{
    pub list_state: ListState,
    pub conversation_list: Vec<T>,
}

// impl ConversationList<T> {
//     pub fn new(conversation_list: Vec<T>) -> Self {
//         ConversationList {
//             list_state: ListState::default(),
//             conversation_list,
//         }
//     }
// }

pub fn render_windows(rx: &mpsc::Receiver<InputEvent<crossterm::event::KeyEvent>>) -> Result<(), Box<dyn std::error::Error>>{
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let config = crate::parse_config().expect("Parse config expect");
    let oauth_token = config["oauth_token"].as_str().expect("OAuth token is not a string");

    let mut active_window_item = MenuItem::Channels;
    let mut focus_window_item = MenuItem::None;

    // Create lists for channels and users
    let mut team_list = ConversationList{
        list_state: ListState::default(),
        conversation_list: Vec::from(["test team"])
    };
    team_list.list_state.select(Some(0));

    let mut channel_list = ConversationList{
        list_state: ListState::default(),
        conversation_list: channel_interface::get_channel_list(oauth_token).expect("Get channel list expect")
    };
    channel_list.list_state.select(Some(0));

    let mut user_list = ConversationList{
        list_state: ListState::default(),
        conversation_list: user_interface::get_user_list(oauth_token).expect("Get user list expect")
    };
    user_list.list_state.select(Some(0));

    let conversation = Conversation::new(channel_list.conversation_list[0].name.to_string(), channel_list.conversation_list[0].id.to_string());
    let messages_list = messages_interface::get_channel_messages(&channel_list.conversation_list[0].id, oauth_token).expect("Get messages list expect");

    let mut input_reciever = InputReciever::new(rx);

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
                channels::render_teams(&team_list.list_state,
                    matches!(active_window_item, MenuItem::Teams), 
                    matches!(focus_window_item, MenuItem::Teams)),
                channels_chunks[0],
                &mut team_list.list_state);
            rect.render_stateful_widget(
                channels::render_channels(&channel_list,
                    matches!(active_window_item, MenuItem::Channels), 
                    matches!(focus_window_item, MenuItem::Channels)),
                channels_chunks[1],
                &mut channel_list.list_state);
            rect.render_stateful_widget(
                channels::render_users(&user_list,
                    matches!(active_window_item, MenuItem::Users), 
                    matches!(focus_window_item, MenuItem::Users)),
                channels_chunks[2],
                &mut user_list.list_state);


            // Render messages and messages input
            let messages_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [Constraint::Percentage(90),
                            Constraint::Percentage(10)].as_ref(),
                        )
                        .split(root_chunks[1]);

            rect.render_widget(conversation.render_messages(&messages_list), messages_chunks[0]);
            rect.render_widget(messages::render_messages_input(matches!(active_window_item, MenuItem::Input)), messages_chunks[1]);
        })?;

        let event = input_reciever.handle_input(
            &mut active_window_item, 
            &mut focus_window_item,
            &mut channel_list,
            &mut user_list)
            .expect("Input expect");

        if matches!(event, InputEvent::Quit){
            disable_raw_mode()?;
            terminal.show_cursor()?;
            break;
        }
    }

    Ok(())
}
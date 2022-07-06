// use chrono::prelude::*;
use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
// use rand::{distributions::Alphanumeric, prelude::*};
use serde::{Deserialize, Serialize, de::Expected};
use serde_json::Value;
use std::{rc::{Rc, Weak}, borrow::BorrowMut};
use std::fs;
use std::io;
use std::sync::mpsc;
use tui::{
    backend::{CrosstermBackend, Backend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Frame,
    Terminal,
};

use crate::{InputEvent, messages::Conversation};
use crate::channels;
use crate::home;
use crate::messages;
use crate::input_reciever::{InputReciever};
use crate::slack_interface::{user_interface, channel_interface, messages_interface};
use crate::observer::{Observer, Event};


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

impl ConversationList<channel_interface::Channel> {
    pub fn get_conversation_id(&self) -> Option<String> {
        if let Some(selected) = self.list_state.selected() {
            Some(self.conversation_list[selected].id.clone())
        } else {
            None
        }
    }
}

impl ConversationList<user_interface::User> {
    pub fn get_conversation_id(&self) -> Option<String> {
        if let Some(selected) = self.list_state.selected() {
            Some(self.conversation_list[selected].id.clone())
        } else {
            None
        }
    }
}

pub fn render_windows(rx: &mpsc::Receiver<InputEvent<crossterm::event::KeyEvent>>) -> Result<(), Box<dyn std::error::Error>>{
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::new(&rx);
    // let app = Rc::new(RefCell::new(App::new(&rx)));

    loop {
        // let app = app.borrow_mut();
        terminal.draw(|rect| draw_ui(rect, &mut app).expect("draw ui expect"));
        let event = app.input_reciever.handle_input(
            &mut app.active_window_item, 
            &mut app.focus_window_item,
            &mut app.channel_list,
            &mut app.user_list)
            .expect("Input expect");

        if matches!(event, InputEvent::Quit){
            disable_raw_mode()?;
            terminal.show_cursor()?;
            break;
        }
    }

    Ok(())
}

pub struct App<'a>{
    config: Value,
    oauth_token: String,
    active_window_item: MenuItem,
    focus_window_item: MenuItem,
    team_list: ConversationList<&'a str>,
    channel_list: ConversationList<channel_interface::Channel>,
    user_list: ConversationList<user_interface::User>,
    conversation: Conversation,
    messages_list: Vec<messages_interface::Message>,
    input_reciever: InputReciever<'a>,
}

impl<'a> App<'a> {
    pub fn new(rx: &'a mpsc::Receiver<InputEvent<crossterm::event::KeyEvent>>) -> Self {
        let config = crate::parse_config().expect("Parse config expect");
        let oauth_token = &config["oauth_token"].as_str()
            .expect("OAuth token is not a string").to_string();
        let mut channel_list = ConversationList{
            list_state: ListState::default(),
            conversation_list: channel_interface::get_channel_list(
                &oauth_token
            ).expect("Get channel list expect")
        };
        channel_list.list_state.select(Some(0));
        let channel_name = &channel_list.conversation_list[0].name.to_string();
        let channel_id = &channel_list.conversation_list[0].id.to_string();

        let mut team_list = ConversationList{
            list_state: ListState::default(),
            conversation_list: Vec::from(["test team"])
        };
        team_list.list_state.select(Some(0));
        let mut user_list = ConversationList{
            list_state: ListState::default(),
            conversation_list: user_interface::get_user_list(oauth_token).expect("Get user list expect")
        };
        user_list.list_state.select(Some(0));

        Self { 
            config: config,
            oauth_token: oauth_token.to_string(),
            active_window_item: MenuItem::Channels,
            focus_window_item: MenuItem::None,
            team_list: team_list,
            channel_list: channel_list,
            user_list: user_list,
            conversation: Conversation::new(
                channel_name.to_string(),
                channel_id.to_string()
            ),
            messages_list: messages_interface::get_channel_messages(
                &channel_id,
                &oauth_token
            ).expect("Get messages list expect"),
            input_reciever: InputReciever::new(rx), // TODO: Don't handle input during draw
        }
    }
}

pub fn draw_ui<B: Backend>(rect: &mut Frame<B>, app: &mut App<'_>) -> Result<(), Box<dyn std::error::Error>> {
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
        channels::render_teams(&app.team_list.list_state,
            matches!(app.active_window_item, MenuItem::Teams), 
            matches!(app.focus_window_item, MenuItem::Teams)),
        channels_chunks[0],
        &mut app.team_list.list_state);
    rect.render_stateful_widget(
        channels::render_channels(&app.channel_list,
            matches!(app.active_window_item, MenuItem::Channels), 
            matches!(app.focus_window_item, MenuItem::Channels)),
        channels_chunks[1],
        &mut app.channel_list.list_state);
    rect.render_stateful_widget(
        channels::render_users(&app.user_list,
            matches!(app.active_window_item, MenuItem::Users), 
            matches!(app.focus_window_item, MenuItem::Users)),
        channels_chunks[2],
        &mut app.user_list.list_state);


    // Render messages and messages input
    let messages_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [Constraint::Percentage(90),
                    Constraint::Percentage(10)].as_ref(),
                )
                .split(root_chunks[1]);

    rect.render_widget(
        messages::render_messages(
            &app.conversation,
            &app.messages_list), 
        messages_chunks[0]);
    rect.render_widget(
        messages::render_messages_input(
            matches!(app.active_window_item, MenuItem::Input)), 
        messages_chunks[1]);

    Ok(())
}
use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
// use rand::{distributions::Alphanumeric, prelude::*};
use serde::{Deserialize, Serialize, de::Expected};
use serde_json::Value;
use std::{rc::{Rc, Weak}, borrow::{BorrowMut, Borrow}};
use std::fs;
use std::cell::RefCell;
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

use crate::{InputEvent, messages::Conversation, input_reciever};
use crate::channels;
use crate::home;
use crate::messages;
use crate::user_interface::User;
use crate::channel_interface::Channel;
use crate::input_reciever::{InputReciever};
use crate::slack_interface::{user_interface, channel_interface, messages_interface};
use crate::ui;

#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ActiveBlock {
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
impl From<ActiveBlock> for usize {
    fn from(item: ActiveBlock) -> usize {
        match item {
            ActiveBlock::None => 0,
            ActiveBlock::Messages => 1,
            ActiveBlock::Input => 2,
            ActiveBlock::Users => 3,
            ActiveBlock::Channels => 4,
            ActiveBlock::Teams => 5,
            ActiveBlock::Search => 6,
            ActiveBlock::Home => 7,
        }
    }
}

pub struct App<'a>{
    pub config: Value,
    pub oauth_token: String,
    pub active_block: ActiveBlock,
    pub hovered_block: ActiveBlock,
    pub team_list: Vec<&'a str>,
    pub channel_list: Vec<channel_interface::Channel>,
    pub user_list: Vec<user_interface::User>,
    pub selected_team_index: Option<usize>,
    pub selected_channel_index: Option<usize>,
    pub selected_user_index: Option<usize>,
    // conversation: Conversation,
    pub messages_list: Vec<messages_interface::Message>,
    // input_reciever: InputReciever<'a>,
}

impl<'a> App<'a> {
    pub fn new(rx: &'a mpsc::Receiver<InputEvent<crossterm::event::KeyEvent>>) -> Self {
        let config = crate::parse_config().expect("Parse config expect");
        let oauth_token = &config["oauth_token"].as_str()
            .expect("OAuth token is not a string").to_string();

        // let mut channel_list = ConversationList::<Channel>::new(
        //     channel_interface::get_channel_list(
        //         &oauth_token).expect("Get channel list expect")
        // );

        // let mut user_list = ConversationList::<User>::new(
        //     user_interface::get_user_list(
        //         &oauth_token).expect("Get user list expect")
        // );

        // TODO: Add constructor for team conversations
        // let mut team_list = ConversationList{
        //     list_state: ListState::default(),
        //     conversation_list: Vec::from(["test team"])
        // };
        // team_list.list_state.select(Some(0));
        // let channel_id = &channel_list.conversation_list[0].id.to_string();

        Self { 
            config: config,
            oauth_token: oauth_token.to_string(),
            active_block: ActiveBlock::Channels,
            hovered_block: ActiveBlock::None,
            team_list: Vec::from(["test team"]),
            // TODO:: Move get channels, users, teams and messages outside of constructor
            channel_list: Vec::from(channel_interface::get_channel_list(
                &oauth_token).expect("Get channel list expect")),
            user_list: Vec::from(user_interface::get_user_list(
                &oauth_token).expect("Get user list expect")),
            selected_team_index: None,
            selected_channel_index: None,
            selected_user_index: None,
            messages_list: Vec::new(),
            // input_reciever: InputReciever::new(rx), 
        }
    }
}

pub fn start_ui(rx: &mpsc::Receiver<InputEvent<crossterm::event::KeyEvent>>)
-> Result<(), Box<dyn std::error::Error>>{
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::new(rx);
    let mut input_reciever = InputReciever::new(rx);

    loop{
        terminal.draw(|rect| 
            ui::draw_ui(rect,
                &app).expect("Draw UI"),
        ).expect("draw ui expect");

        let event = input_reciever.handle_input(
            &mut app)
            .expect("Input expect");

        if matches!(event, InputEvent::Quit){
            disable_raw_mode()?;
            terminal.show_cursor()?;
            break;
        }
    }

    Ok(())
}
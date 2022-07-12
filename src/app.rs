use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
// use rand::{distributions::Alphanumeric, prelude::*};
use serde_json::Value;
use std::io;
use std::sync::mpsc;
use tui::{
    backend::{CrosstermBackend, Backend},
    Terminal,
};

use crate::{InputEvent, };
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
    pub active_conversation_id: Option<String>,
    pub active_conversation_name: Option<String>,
}

impl<'a> App<'a> {
    pub fn new(rx: &'a mpsc::Receiver<InputEvent<crossterm::event::KeyEvent>>) -> Self {
        let config = crate::parse_config().expect("Parse config expect");
        let oauth_token = &config["oauth_token"].as_str()
            .expect("OAuth token is not a string").to_string();

        Self { 
            config: config,
            oauth_token: oauth_token.to_string(),
            active_block: ActiveBlock::None,
            hovered_block: ActiveBlock::Channels,
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
            active_conversation_id: None,
            active_conversation_name: None,
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
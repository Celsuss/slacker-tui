use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
    cursor::{MoveTo}, ExecutableCommand,
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
    pub messages_list: Vec<messages_interface::Message>,
    pub active_conversation_id: Option<String>,
    pub active_conversation_name: Option<String>,
    pub input: Vec<char>,
    pub input_idx: usize,
    pub input_cursor_position: u16,
    pub input_cursor_horizontal_offset: u16,
    pub input_cursor_vertical_offset: u16,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
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
            input: vec![],
            input_idx: 0,
            input_cursor_position: 0,
            input_cursor_horizontal_offset: 0,
            input_cursor_vertical_offset: 0,
        }
    }

    pub fn change_conversation(&mut self, conversation_id: &String, conversation_name: &String){
        // TODO: Make sure not to change to the same conversation

        self.active_conversation_id = Some(conversation_id.to_owned());
        self.active_conversation_name = Some(conversation_name.to_owned());

        // TODO: Make sure to be able to get user conversation as well
        self.messages_list = messages_interface::get_channel_messages(
            conversation_id, &self.oauth_token).expect("Get messages expect");
    }
}

pub fn start_ui(rx: &mpsc::Receiver<InputEvent<crossterm::event::KeyEvent>>)
-> Result<(), Box<dyn std::error::Error>>{
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::new();

    let mut input_reciever = InputReciever::new(rx);

    loop{
        // Draw UI
        terminal.draw(|rect| 
            ui::draw_ui(rect,
                &app).expect("Draw UI"),
        ).expect("draw ui expect");

        // Handle input
        let event = input_reciever.handle_input(
            &mut app)
            .expect("Input expect");

       update_cursor(&mut terminal, &mut app)?;

        // Handle exit event
        if matches!(event, InputEvent::Quit){
            disable_raw_mode()?;
            terminal.show_cursor()?;
            break;
        }
    }

    Ok(())
}

pub fn update_cursor(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // Set cursor offset
    app.input_cursor_horizontal_offset = ui::CHANNELS_WIDTH + (ui::MARGIN * 2);
    let bottom = terminal.size()?.bottom() as f64;  // Convert to f64 to to do floating point math
    app.input_cursor_vertical_offset = (bottom * (ui::MESSAGES_HEIGHT_PERCENTAGE as f64/100.0)) as u16; // + (ui::MARGIN * 6);

    // Set cursor position
    if app.active_block == ActiveBlock::Input {
        terminal.show_cursor()?;
        terminal.backend_mut().execute(
            MoveTo(
                app.input_cursor_position + app.input_cursor_horizontal_offset,
                app.input_cursor_vertical_offset))
            .expect("Set cursor position expect");
    }
    else{
        terminal.hide_cursor()?;
    }
    Ok(())
}
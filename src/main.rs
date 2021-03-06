// use chrono::prelude::*;
use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
// use rand::{distributions::Alphanumeric, prelude::*};
use serde::{Deserialize, Serialize, de::Expected};
use serde_json::Value;
use std::fs;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

mod home;
mod messages;
mod input_reciever;
mod slack_interface;
mod ui;
mod util;
mod app;

use slack_interface::{user_interface, channel_interface, messages_interface};

// Input events
pub enum InputEvent<T> { 
    Input(T),
    Tick,
    Quit,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // For testing without setting terminal to raw mode
    // let config = crate::parse_config().expect("Parse config expect");
    // let oauth_token = config["oauth_token"].as_str().expect("OAuth token is not a string");
    // let user_list = user_interface::get_user_list(oauth_token).expect("Get user list expect");
    // let channel_list = channel_interface::get_channel_list(oauth_token).expect("Get channel list expect");
    // let messages_list = messages_interface::get_channel_messages(&channel_list[0].id, oauth_token).expect("Get messages list expect");

    // let send_message = messages_interface::send_channel_message(&channel_list[0].name, oauth_token).expect("Send message expect");

    // println!("Conversation : {}, {}", channel_list[0].name, channel_list[0].id);
    // println!("Success");
    // return Ok(());  


    // enable_raw_mode().expect("can run in raw mode");
    let (tx, rx) = mpsc::channel(); // Create a channel for sending and receiving events
    let tick_rate = Duration::from_millis(200); // Tick rate in milliseconds

    // Create a thread for handling input events
    thread::spawn(move || input_listen(&tx, &tick_rate));

    enable_raw_mode().expect("Enable raw mode expect");
    app::start_ui(&rx).expect("Start ui expect");

    Ok(())
}

fn parse_config() -> Result<Value, Box<dyn std::error::Error>> {
    let config_file = fs::read_to_string("config.json")?;
    let config: Value = serde_json::from_str(&config_file)?;
    Ok(config)
}

fn input_listen(tx: &mpsc::Sender<InputEvent<KeyEvent>>, tick_rate: &Duration) -> Result<(), io::Error> {
    let mut last_tick = Instant::now();
    loop {
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // wait (timeout) for user input, if no input then send tick event
        if event::poll(timeout).expect("poll works") {
            if let CEvent::Key(key) = event::read().expect("can read events") {
                tx.send(InputEvent::Input(key)).expect("can send events");
            }
        }

        if last_tick.elapsed() >= *tick_rate {
            if let Ok(_) = tx.send(InputEvent::Tick) {
                last_tick = Instant::now();
            }
        }
    }
}
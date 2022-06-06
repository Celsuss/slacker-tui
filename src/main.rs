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
use std::thread;
use std::time::{Duration, Instant};
use thiserror::Error;
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

mod channels;
mod windows;
mod home;
mod messages;

// Input events
pub enum Event<T> { 
    Input(T),
    Tick,
    Quit,
    Change(T)
}

// TUI Menu structure
#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Home,
    Channels,
    Messages,
    Input,
    Search,
}

// Convert MenuItem to usize, will be used to
// highlight the current menu item using Tabs in TUI component
impl From<MenuItem> for usize {
    fn from(item: MenuItem) -> usize {
        match item {
            MenuItem::Home => 0,
            MenuItem::Channels => 1,
            MenuItem::Messages => 2,
            MenuItem::Input => 3,
            MenuItem::Search => 4,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // enable_raw_mode().expect("can run in raw mode");
    let (tx, rx) = mpsc::channel(); // Create a channel for sending and receiving events
    let tick_rate = Duration::from_millis(200); // Tick rate in milliseconds

    // Create a thread for handling input events
    thread::spawn(move || input_listen(&tx, &tick_rate));

    enable_raw_mode().expect("can run in raw mode");
    windows::render_windows(&rx).expect("can render windows");

    Ok(())
}

fn input_listen(tx: &mpsc::Sender<Event<KeyEvent>>, tick_rate: &Duration) -> Result<(), io::Error> {
    let mut last_tick = Instant::now();
    loop {
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // wait (timeout) for user input, if no input then send tick event
        if event::poll(timeout).expect("poll works") {
            if let CEvent::Key(key) = event::read().expect("can read events") {
                tx.send(Event::Input(key)).expect("can send events");
            }
        }

        if last_tick.elapsed() >= *tick_rate {
            if let Ok(_) = tx.send(Event::Tick) {
                last_tick = Instant::now();
            }
        }
    }
}
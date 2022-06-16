// use chrono::prelude::*;
use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use tui::{
    Terminal, widgets::ListState,
};
use std::sync::mpsc;

use crate::windows::MenuItem;
use crate::{Event};

pub fn recieve_input(rx: &mpsc::Receiver<Event<crossterm::event::KeyEvent>>, active_window_item: &mut MenuItem, focus_window_item: &mut MenuItem,
                    channel_list_state: &mut ListState, user_list_state: &mut ListState) -> Result<Event<()>, Box<dyn std::error::Error>>{
     // Receive event from input thread
     match rx.recv()? {
        Event::Input(event) => match event {
            // Quit software is user presses 'q'
            KeyEvent{ code: KeyCode::Char('q'), modifiers: KeyModifiers::NONE} => {
                return Ok(Event::Quit);
            }
            // Select window to focus on
            KeyEvent { code: KeyCode::Enter, modifiers: KeyModifiers::NONE } => {
                focus_window_item.clone_from(active_window_item);
            }
            // Deselect focused window
            KeyEvent { code: KeyCode::Esc, modifiers: KeyModifiers::NONE } => {
                focus_window_item.clone_from(&MenuItem::None);
            }

            // Handle movement between windows and lists
            KeyEvent{ code: KeyCode::Up, modifiers: KeyModifiers::NONE} => {
                match focus_window_item {
                    MenuItem::Channels => {
                        update_list_state(channel_list_state, KeyCode::Up);
                    },
                    MenuItem::Users => {
                        update_list_state(user_list_state, KeyCode::Up);
                    },
                    MenuItem::None => {
                        move_up(active_window_item);
                    }
                    _ => {}
                }
            }
            KeyEvent{ code: KeyCode::Right, modifiers: KeyModifiers::NONE} => {
                *focus_window_item = MenuItem::None;
                move_right(active_window_item);
            }
            KeyEvent{ code: KeyCode::Down, modifiers: KeyModifiers::NONE} => {
                match focus_window_item {
                    MenuItem::Channels => {
                        update_list_state(channel_list_state, KeyCode::Down);
                    },
                    MenuItem::Users => {
                        update_list_state(user_list_state, KeyCode::Down);
                    },
                    MenuItem::None => {
                        move_down(active_window_item);
                    }
                    _ => {}
                }
            }
            KeyEvent{ code: KeyCode::Left, modifiers: KeyModifiers::NONE} => {
                *focus_window_item = MenuItem::None;
                move_left(active_window_item);
            }
            _ => {}
        },
        _ => {},
    }
    Ok(Event::Tick)
}

fn update_list_state(list_state: &mut ListState, code: KeyCode){
    // TODO: Make sure to not select an object outside of list size
    match code {
        KeyCode::Up => {
            if let Some(selected) = list_state.selected() {
                list_state.select(Some(selected - 1));
            }
        }
        KeyCode::Down => {
            if let Some(selected) = list_state.selected() {
                list_state.select(Some(selected + 1));
            }
        }
        _ => {}
    }
}

fn move_up(active_window_item: &mut MenuItem) {
    match active_window_item {
        MenuItem::Channels => {
            *active_window_item = MenuItem::Teams;
        }
        MenuItem::Users => {
            *active_window_item = MenuItem::Channels;
        }
        MenuItem::Input => {
            *active_window_item = MenuItem::Channels;
        }
        _ => {}
    }
}

fn move_right(active_window_item: &mut MenuItem) {
    *active_window_item = MenuItem::Input;
}

fn move_down(active_window_item: &mut MenuItem) {
    match active_window_item {
        MenuItem::Teams => {
            *active_window_item = MenuItem::Channels;
        }
        MenuItem::Channels => {
            *active_window_item = MenuItem::Users;
        }
        _ => {}
    }
}

fn move_left(active_window_item: &mut MenuItem) {
    match active_window_item {
        MenuItem::Input => {
            *active_window_item = MenuItem::Users;
        }
        _ => {}
    }
}
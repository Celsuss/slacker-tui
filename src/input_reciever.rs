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
                    channel_list_state: &mut ListState, channel_list_size: usize, user_list_state: &mut ListState, user_list_size: usize) -> Result<Event<()>, Box<dyn std::error::Error>>{
     // Receive event from input thread
     match rx.recv()? {
        Event::Input(event) => match event {
            // Priority key presses
            // Quit software is user presses 'q'
            KeyEvent{ code: KeyCode::Char('q'), modifiers: KeyModifiers::NONE} => {
                return Ok(Event::Quit);
            }
            // Deselect focused window
            KeyEvent { code: KeyCode::Esc, modifiers: KeyModifiers::NONE } => {
                focus_window_item.clone_from(&MenuItem::None);
            }
            _ => {
                match focus_window_item {
                    MenuItem::Channels => {
                        update_list_state(channel_list_state, channel_list_size, event.code);
                    },
                    MenuItem::Users => {
                        update_list_state(user_list_state, user_list_size, event.code);
                    },
                    _ => {
                        navigate_windows(event.code, active_window_item, focus_window_item);
                    }
                }
            }

            
            
            
        },
        _ => {},
    }
    Ok(Event::Tick)
}

fn update_list_state(list_state: &mut ListState, list_size: usize, code: KeyCode){
    match code {
        KeyCode::Up => {
            if let Some(selected) = list_state.selected() {
                if selected > 0 {
                    list_state.select(Some(selected - 1));
                }
            }
        }
        KeyCode::Down => {
            if let Some(selected) = list_state.selected() {
                if selected < list_size -1  {
                    list_state.select(Some(selected + 1));
                }
            }
        }
        _ => {}
    }
}

fn update_conversation(list_state: &mut ListState){

}

fn navigate_windows(code: KeyCode, active_window_item: &mut MenuItem, focus_window_item: &mut MenuItem){
    match code{
        KeyCode::Up => {
            move_up(active_window_item);
        }
        KeyCode::Down => {
            move_down(active_window_item);
        }
        KeyCode::Left => {
            move_left(active_window_item);
        }
        KeyCode::Right => {
            move_right(active_window_item);
        }
        KeyCode::Enter => {
            focus_window_item.clone_from((active_window_item));
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
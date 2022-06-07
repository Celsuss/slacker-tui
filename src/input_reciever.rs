// use chrono::prelude::*;
use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use tui::{
    Terminal,
};
use std::sync::mpsc;

use crate::windows::MenuItem;
use crate::{Event};

pub fn recieve_input(rx: &mpsc::Receiver<Event<crossterm::event::KeyEvent>>, active_window_item: &mut MenuItem) -> Result<(), Box<dyn std::error::Error>>{
     // Receive event from input thread
     match rx.recv()? {
        Event::Input(event) => match event {
            // KeyEvent{ code: KeyCode::Char('q'), modifiers: KeyModifiers::NONE} => {
            //     disable_raw_mode()?;
            //     terminal.show_cursor()?;
            //     break;
            // }

            KeyEvent{ code: KeyCode::Up, modifiers: KeyModifiers::NONE} => {
                move_up(active_window_item);           
            }
            KeyEvent{ code: KeyCode::Right, modifiers: KeyModifiers::NONE} => {
                move_right(active_window_item);
            }
            KeyEvent{ code: KeyCode::Down, modifiers: KeyModifiers::NONE} => {
                move_down(active_window_item);
            }
            KeyEvent{ code: KeyCode::Left, modifiers: KeyModifiers::NONE} => {
                move_left(active_window_item);
                
            }
            _ => {}
        },
        Event::Tick => {},
        Event::Quit | Event::Change(_) => todo!(),
    }
    Ok(())
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
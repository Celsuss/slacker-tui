// use chrono::prelude::*;
use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use tui::{
    Terminal, widgets::ListState,
};
use std::sync::mpsc;

use crate::windows::{MenuItem, ConversationList};
use crate::{InputEvent};
use crate::slack_interface::{user_interface::User, channel_interface::Channel};
use crate::observer::{Notifier, Event};

pub struct InputReciever<'a> {
    rx: &'a mpsc::Receiver<InputEvent<crossterm::event::KeyEvent>>,
    notifier: Notifier,
}

impl<'a> InputReciever<'a> {
    pub fn new(
        rx: &'a mpsc::Receiver<InputEvent<crossterm::event::KeyEvent>>,
    ) -> Self {
        InputReciever {
            rx,
            notifier: Notifier::new(),
        }
    }

    pub fn handle_input(&mut self, active_window_item: &mut MenuItem, focus_window_item: &mut MenuItem,
        channel_list: &mut ConversationList<Channel>, user_list: &mut ConversationList<User>) -> Result<InputEvent<()>, Box<dyn std::error::Error>>{
        // Receive event from input thread
        match self.rx.recv()? {
            InputEvent::Input(event) => match event {
                // Priority key presses
                // Quit software is user presses 'q'
                KeyEvent{ code: KeyCode::Char('q'), modifiers: KeyModifiers::NONE} => {
                    return Ok(InputEvent::Quit);
                }
                // Deselect focused window
                KeyEvent { code: KeyCode::Esc, modifiers: KeyModifiers::NONE } => {
                    focus_window_item.clone_from(&MenuItem::None);
                }
                _ => {
                    match focus_window_item {
                        MenuItem::Channels => {
                            self.update_list_state(channel_list, event.code)
                                .expect("Update channel list state expect");
                        },
                        MenuItem::Users => {
                            self.update_list_state(user_list, event.code)
                                .expect("Update user list state expect");
                        },
                        _ => {
                            self.navigate_windows(event.code, active_window_item, focus_window_item);
                        }
                    }
                }

            },
            _ => {},
        }
        Ok(InputEvent::Tick)
    }

    fn update_list_state<T>(&self, list: &mut ConversationList<T>, code: KeyCode) -> Result<InputEvent<()>, Box<dyn std::error::Error>>{ 
        match code {
            KeyCode::Up => {
                if let Some(selected) = list.list_state.selected() {
                    if selected > 0 {
                        list.list_state.select(Some(selected - 1));
                    }
                }
            }
            KeyCode::Down => {
                if let Some(selected) = list.list_state.selected() {
                    if selected < list.conversation_list.len() - 1  {
                        list.list_state.select(Some(selected + 1));
                    }
                }
            }
            KeyCode::Enter => {
                self.notifier.notify_observers(Event::ChangeConversation(("test").to_string()));
            }
            _ => {}
        }
        Ok(InputEvent::Tick)
    }
    
    fn navigate_windows(&self, code: KeyCode, active_window_item: &mut MenuItem, focus_window_item: &mut MenuItem){
        match code{
            KeyCode::Up => {
                self.move_up(active_window_item);
            }
            KeyCode::Down => {
                self.move_down(active_window_item);
            }
            KeyCode::Left => {
                self.move_left(active_window_item);
            }
            KeyCode::Right => {
                self.move_right(active_window_item);
            }
            KeyCode::Enter => {
                focus_window_item.clone_from((active_window_item));
            }
            _ => {}
        }
    }
    
    fn move_up(&self, active_window_item: &mut MenuItem) {
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
    
    fn move_right(&self, active_window_item: &mut MenuItem) {
        *active_window_item = MenuItem::Input;
    }
    
    fn move_down(&self, active_window_item: &mut MenuItem) {
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
    
    fn move_left(&self, active_window_item: &mut MenuItem) {
        match active_window_item {
            MenuItem::Input => {
                *active_window_item = MenuItem::Users;
            }
            _ => {}
        }
    }
}
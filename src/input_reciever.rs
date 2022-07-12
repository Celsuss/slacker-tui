// use chrono::prelude::*;
use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use tui::{
    Terminal, widgets::ListState,
};
use std::sync::mpsc;

use crate::app::{App, ActiveBlock};
use crate::{InputEvent};
use crate::slack_interface::{user_interface::User, channel_interface::Channel};
use crate::util;

pub struct InputReciever<'a> {
    rx: &'a mpsc::Receiver<InputEvent<crossterm::event::KeyEvent>>,
}

impl<'a> InputReciever<'a> {
    pub fn new(
        rx: &'a mpsc::Receiver<InputEvent<crossterm::event::KeyEvent>>,
    ) -> Self {
        InputReciever {
            rx,
        }
    }

    pub fn handle_input(&mut self, app: &mut App) -> Result<InputEvent<()>, Box<dyn std::error::Error>>{
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
                    // TODO: Move this to a function
                    app.active_block.clone_from(&ActiveBlock::None);
                    // TODO: Improve to only use one selected index
                    app.selected_team_index = None;
                    app.selected_channel_index = None;
                    app.selected_user_index = None;
                }
                _ => {
                    match app.active_block {
                        ActiveBlock::Channels => {
                            self.update_list_state(&mut app.selected_channel_index, 
                                &app.channel_list, event.code)
                                .expect("Update channel list state expect");
                            self.select_list_element(app, 
                                app.selected_channel_index, 
                                app.channel_list.iter()
                                    .map(|c| (c.id.clone(), c.name.clone()))
                                    .collect(), 
                                event.code);
                        },
                        ActiveBlock::Users => {
                            self.update_list_state(&mut app.selected_user_index,
                                &app.user_list, event.code)
                                .expect("Update user list state expect");
                            self.select_list_element(app, 
                                app.selected_user_index, 
                                app.user_list.iter()
                                    .map(|u| (u.id.clone(), u.name.clone()))
                                    .collect(),
                                event.code);
                        },
                        ActiveBlock::Teams => {

                        }
                        ActiveBlock::Input => {
                            self.handle_user_intput(app, event.code);
                        }
                        ActiveBlock::None => {
                            // If no window is focused, check if user pressed 'c' to select channel
                            // if event.code == KeyCode::Char('c') {
                            //     app.active_block.clone_from(&ActiveBlock::Channels);
                            // } else if event.code == KeyCode::Char('u') {
                            //     app.active_block.clone_from(&ActiveBlock::Users);
                            // } else if event.code == KeyCode::Char('t') {
                            //     app.active_block.clone_from(&ActiveBlock::Teams);
                            // }

                            // No active block, navigate hovered block
                            self.navigate_windows(event.code, app);
                        }
                        _ => {
                            
                        }
                    }
                }

            },
            _ => {},
        }
        Ok(InputEvent::Tick)
    }

    fn handle_user_intput(&self, app: &mut App, code: KeyCode){
        match code {
            KeyCode::Left => {

            }
            KeyCode::Right => {

            }
            KeyCode::Backspace => {
                if !app.input.is_empty() && app.input_idx > 0 {
                    let c = app.input.remove(app.input_idx - 1);
                    app.input_idx -= 1;
                    app.input_cursor_position -= util::calculate_character_width(c);
                  }
            }
            KeyCode::Char(c) => {
                app.input.insert(app.input_idx, c);
                app.input_idx += 1;
                app.input_cursor_position += util::calculate_character_width(c);
            }
            _ => {}
        }
    }

    fn update_list_state<T>(&self, list_index: &mut Option<usize>,
        list: &Vec<T>, code: KeyCode)
    -> Result<(), Box<dyn std::error::Error>>{ 
        if list.len() == 0 {
            return Ok(());
        }

        match code {
            KeyCode::Up => {
                if let Some(list_index) = list_index {
                    if *list_index > 0 {
                        *list_index -= 1;
                    }
                }
                else{
                    *list_index = Some(0);
                }
            }
            KeyCode::Down => {
                if let Some(list_index) = list_index {
                    if *list_index < list.len() - 1 {
                        *list_index += 1;
                    }
                }
                else{
                    *list_index = Some(0);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn select_list_element(&self, app: &mut App, list_index: Option<usize>, list: Vec<(String, String)>, code: KeyCode){
        if code == KeyCode::Enter {
            if let Some(index) = list_index {
                if index >= list.len() { return; }

                let conversation_id_and_name = list.get(index).unwrap();
                app.change_conversation(
                    &conversation_id_and_name.0, 
                    &conversation_id_and_name.1);
            }
        }
    }
    
    fn navigate_windows(&self, code: KeyCode, app: &mut App){
        match code{
            KeyCode::Up => {
                self.move_up(&mut app.hovered_block);
            }
            KeyCode::Down => {
                self.move_down(&mut app.hovered_block);
            }
            KeyCode::Left => {
                self.move_left(&mut app.hovered_block);
            }
            KeyCode::Right => {
                self.move_right(&mut app.hovered_block);
            }
            KeyCode::Enter => {
                app.active_block.clone_from(&app.hovered_block);
            }
            _ => {}
        }
    }
    
    fn move_up(&self, active_window_item: &mut ActiveBlock) {
        match active_window_item {
            ActiveBlock::Channels => {
                *active_window_item = ActiveBlock::Teams;
            }
            ActiveBlock::Users => {
                *active_window_item = ActiveBlock::Channels;
            }
            ActiveBlock::Input => {
                *active_window_item = ActiveBlock::Channels;
            }
            _ => {}
        }
    }
    
    fn move_right(&self, active_window_item: &mut ActiveBlock) {
        *active_window_item = ActiveBlock::Input;
    }
    
    fn move_down(&self, active_window_item: &mut ActiveBlock) {
        match active_window_item {
            ActiveBlock::Teams => {
                *active_window_item = ActiveBlock::Channels;
            }
            ActiveBlock::Channels => {
                *active_window_item = ActiveBlock::Users;
            }
            _ => {}
        }
    }
    
    fn move_left(&self, active_window_item: &mut ActiveBlock) {
        match active_window_item {
            ActiveBlock::Input => {
                *active_window_item = ActiveBlock::Users;
            }
            _ => {}
        }
    }
}
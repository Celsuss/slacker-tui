// use chrono::prelude::*;
use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs, Wrap,
    },
    Terminal,
};

use crate::slack_interface::{messages_interface::{self, Message}};
use crate::observer::{Observer, Event};

pub struct Conversation{
    conversation_name: String,
    conversation_id: String,
}

impl Conversation{
    pub fn new(conversation_name: String, conversation_id: String) -> Conversation{
        Conversation{
            conversation_name: conversation_name,
            conversation_id: conversation_id,
        }
    }

    pub fn set_conversation_name_and_id(&mut self, name: String, id: String){
        self.conversation_name = name;
        self.conversation_id = id;
    }

}

impl Observer for Conversation{
    fn notify(&self, event: &Event){
        match event {
            Event::ChangeConversation(conversation_id) => {
                println!("Conversation changed to {}", conversation_id);
            }
        }
    }
}

pub fn render_messages<'a>(conversation: &Conversation, messages: &Vec<Message>) -> Paragraph<'a>{
    // Message block
    let messages_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title(format!("Messages - {}", conversation.conversation_name))
        .border_type(BorderType::Plain);

    let items: Vec<_> = messages.iter().rev()
        .map(|message| 
            Spans::from(vec![
                Span::raw("["),
                Span::styled(
                    message.ts.clone(), // TODO: Format time
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("]"),
                Span::raw(" "),
                Span::raw("<"),
                Span::styled(
                    message.username.clone(),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(">"),
                Span::raw(" "),
                Span::styled(
                    message.text.clone(),
                    Style::default(),
                ),
            ])
        ).collect();

    let paragraph = Paragraph::new(items)
        .block(messages_block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    paragraph

}

pub fn render_messages_input<'a>(is_active: bool) -> Paragraph<'a>{
    // TODO: Get text input
    // Get text input
    let text_input = "test input".to_string();

    // Paragraph block
    let paragraph_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if is_active { Color::Magenta } else { Color::White }))
        .style(Style::default().fg(Color::White))
        .title("Input")
        .border_type(BorderType::Plain);

    let input: Paragraph = Paragraph::new(vec![
        Spans::from(vec![
            Span::raw(text_input.clone()),
        ])
    ])
    .alignment(Alignment::Left)
    .block(paragraph_block)
    .wrap(Wrap { trim: true });

    input
}
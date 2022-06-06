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

struct Message{
    name: String,
    message: String,
    date: String,
    time: String,
}

pub fn render_messages<'a>() -> Paragraph<'a>{
    // TODO: Get messages
    // Get messages
    let message_list = vec![
        Message{
            name: "test name".to_string(),
            message: "test message".to_string(),
            date: "test date".to_string(),
            time: "test time".to_string(),
        },
        Message{
            name: "test name 2".to_string(),
            message: "test message 2".to_string(),
            date: "test date 2".to_string(),
            time: "test time 2".to_string(),
        },
    ];

    // Message block
    let messages_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Messages")
        .border_type(BorderType::Plain);

    let items: Vec<_> = message_list.iter()
        .map(|message| 
            Spans::from(vec![
                Span::raw("["),
                Span::styled(
                    message.time.clone(),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("]"),
                Span::raw(" "),
                Span::raw("<"),
                Span::styled(
                    message.name.clone(),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(">"),
                Span::raw(" "),
                Span::styled(
                    message.message.clone(),
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

pub fn render_messages_input<'a>() -> Paragraph<'a>{
    // TODO: Get text input
    // Get text input
    let text_input = "test input".to_string();

    // Paragraph block
    let paragraph_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
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
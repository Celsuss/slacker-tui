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
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};

struct Channel {
    name: String,
    unread_count: usize,
}

pub fn render_channels<'a>(channel_list_state: &ListState) -> List<'a> {
    let channels = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Channels")
        .border_type(BorderType::Plain);

    // TODO: Get channel list
    let channel_list = vec![
        Channel {
            name: "general".to_string(),
            unread_count: 0,
        },
        Channel {
            name: "random".to_string(),
            unread_count: 0,
        },
    ];

    let items: Vec<_> = channel_list.iter()
        .map(|channel| {
            ListItem::new(Spans::from(vec![Span::styled(
                channel.name.clone(),
                Style::default(),
            )]))
        }).collect();

    // let current_channel = channel_list_state.selected().map(|i| channel_list[i]);
    let current_channel = channel_list.get(
        channel_list_state.selected()
            .expect("There should be a selected channel"),
    )
    .expect("There should be a selected channel")
    .clone();

    let list = List::new(items)
        .block(channels)
        .highlight_style(Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    list
}
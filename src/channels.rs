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

use crate::slack_interface::{user_interface::User};

struct Channel {
    name: String,
    unread_count: usize,
}

pub fn render_channels<'a>(channel_list: &Vec<String>, channel_list_state: &ListState, is_active: bool) -> List<'a> {
    let channels = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if is_active { Color::Magenta } else { Color::White }))
        .style(Style::default().fg(Color::White))
        .title("Channels")
        .border_type(BorderType::Plain);

    let items: Vec<_> = channel_list.iter()
        .map(|channel| {
            ListItem::new(Spans::from(vec![Span::styled(
                channel.clone(),
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

pub fn render_teams(team_list_state: &ListState, is_active: bool) -> List<'static> {
    let teams_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if is_active { Color::Magenta } else { Color::White }))
        .style(Style::default().fg(Color::White))
        .title("Teams")
        .border_type(BorderType::Plain);

    // TODO: Get team list
    let team_list = vec![
        "general".to_string(),
        "random".to_string(),
    ];

    let items: Vec<_> = team_list.iter()
        .map(|team| {
            ListItem::new(Spans::from(vec![Span::styled(
                team.clone(),
                Style::default(),
            )]))
        }).collect();

    // let current_channel = channel_list_state.selected().map(|i| channel_list[i]);
    let current_channel = team_list.get(
        team_list_state.selected()
            .expect("There should be a selected channel"),
    )
    .expect("There should be a selected channel")
    .clone();

    let list = List::new(items)
        .block(teams_block)
        .highlight_style(Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    list
}

pub fn render_users(user_list: &Vec<User>, user_list_state: &ListState, is_active: bool) -> List<'static> {
    let users_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if is_active { Color::Magenta } else { Color::White }))
        .style(Style::default().fg(Color::White))
        .title("Users")
        .border_type(BorderType::Plain);

    let items: Vec<_> = user_list.iter()
        .map(|user| {
            ListItem::new(Spans::from(vec![Span::styled(
                user.name.clone(),
                Style::default(),
            )]))
        }).collect();

    // let current_channel = channel_list_state.selected().map(|i| channel_list[i]);
    let current_channel = user_list.get(
        user_list_state.selected()
            .expect("There should be a selected channel"),
    )
    .expect("There should be a selected channel")
    .clone();

    let list = List::new(items)
        .block(users_block)
        .highlight_style(Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    list
}
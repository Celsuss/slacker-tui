use tui::{
    backend::{CrosstermBackend, Backend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Frame,
    Terminal,
};

use crate::util;
use crate::windows::App;

pub fn draw_ui<B: Backend>(frame: &mut Frame<B>, app: &App<'_>)
-> Result<(), Box<dyn std::error::Error>> {
    let size = frame.size();
    let root_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Length(20),   // Channels
                Constraint::Min(2),       // Messages
            ]
            .as_ref(),
        )
        .split(size);
    
    // Render teams, channels and users
    draw_lists(frame, app, root_chunk[0]);

    draw_conversation(frame, app, root_chunk[1]);

    Ok(())
}

pub fn draw_lists<B>(frame: &mut Frame<B>, app: &App<'_>, chunk: Rect)
where
    B: Backend{
    let channel_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),     // Teams
                Constraint::Percentage(55),     // Channels
                Constraint::Percentage(35),     // Users
            ]
            .as_ref(),
        )
        .split(chunk);

    // draw_teams(frame, app, channel_chunks[0]);
    draw_channels(frame, app, channel_chunks[1]);
    // draw_users(frame, app, channel_chunks[2]);
}

pub fn draw_channels<B>(frame: &mut Frame<B>, app: &App<'_>, chunk: Rect)
where
B: Backend{
    // Get channels

    let items: Vec<_> = app.channel_list.conversation_list.iter()
        .map(|channel| {
            ListItem::new(Spans::from(vec![Span::styled(
                channel.name.clone(),
                Style::default(),
            )]))
        }).collect();
}

pub fn draw_conversation<B>(frame: &mut Frame<B>, app: &App<'_>, chunk: Rect)
where
B: Backend{
    let conversation_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [Constraint::Percentage(90),
            Constraint::Percentage(10)].as_ref(),
        )
        .split(chunk);

    // draw_conversation(frame, app, conversation_chunks[0])?;
    // draw_conversation_input(frame, app, conversation_chunks[1])?;
}

pub fn draw_selectable_list<B, S>(frame: &mut Frame<B>, app: &App<'_>, chunk: Rect,
    title: &str, items: &[S], highlight_state: (bool, bool), selected_index: Option<usize>)
where
B: Backend,
S: std::convert::AsRef<str>{
    let mut state = ListState::default();
    state.select(selected_index);

    let list_items: Vec<ListItem> = items
        .iter()
        .map(|i| ListItem::new(Span::raw(i.as_ref())))
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_type(BorderType::Plain)
        .style(Style::default().fg(Color::White))
        .border_style(util::get_color(highlight_state));

    let list = List::new(list_items)
        .block(block)
        .highlight_style(Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_stateful_widget(list, chunk, &mut state);
}
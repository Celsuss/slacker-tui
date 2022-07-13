use tui::{
    backend::{CrosstermBackend, Backend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem,
        ListState, Paragraph, Row, Table, Tabs, Wrap,
    },
    Frame,
};

use crate::util;
use crate::app::{
    App, ActiveBlock,
};

pub const MARGIN: u16 = 1;
pub const MESSAGES_HEIGHT_PERCENTAGE: u16 = 90;
pub const CHANNELS_WIDTH: u16 = 20;

pub fn draw_ui<B: Backend>(frame: &mut Frame<B>, app: &App<'_>)
-> Result<(), Box<dyn std::error::Error>> {
    let size = frame.size();
    let root_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .margin(MARGIN)
        .constraints(
            [
                Constraint::Length(CHANNELS_WIDTH),   // Channels
                Constraint::Min(2),       // Messages
            ]
            .as_ref(),
        )
        .split(size);
    
    // Render teams, channels and users
    draw_lists(frame, app, root_chunk[0]);

    // Render messages and messages input
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

    draw_teams(frame, app, channel_chunks[0]);
    draw_channels(frame, app, channel_chunks[1]);
    draw_users(frame, app, channel_chunks[2]);
}

pub fn draw_teams<B>(frame: &mut Frame<B>, app: &App<'_>, chunk: Rect)
where
B: Backend{
    let title = "Teams";

    // Temp teams
    let team_list = vec!["Imagimob", "Other"];
    // Get Teams
    let items: Vec<_> = team_list.iter()
        .map(|team| team.to_owned())
        .collect();

    let highlight_state = (
        app.active_block == ActiveBlock::Teams,
        app.hovered_block == ActiveBlock::Teams,
    );

    draw_selectable_list(frame, app, chunk, title, &items,
        highlight_state, &app.selected_team_index);
}

pub fn draw_channels<B>(frame: &mut Frame<B>, app: &App<'_>, chunk: Rect)
where
B: Backend{
    let title = "Channels";

    // Get channels
    let items: Vec<_> = app.channel_list.iter()
        .map(|item| item.name.to_owned())
        .collect();

    // let current_route = app.get_current_route();
    let highlight_state = (
        app.active_block == ActiveBlock::Channels,
        app.hovered_block == ActiveBlock::Channels,
    );

    draw_selectable_list(frame, app, chunk, title, &items,
        highlight_state, &app.selected_channel_index);
}

pub fn draw_users<B>(frame: &mut Frame<B>, app: &App<'_>, chunk: Rect)
where
B: Backend{
    let title = "Users";

    // Get users
    let items: Vec<_> = app.user_list.iter()
        .map(|item| item.name.to_owned())
        .collect();

    // let current_route = app.get_current_route();
    let highlight_state = (
        app.active_block == ActiveBlock::Users,
        app.hovered_block == ActiveBlock::Users,
    );

    draw_selectable_list(frame, app, chunk, title, &items,
        highlight_state, &app.selected_user_index);
}

pub fn draw_conversation<B>(frame: &mut Frame<B>, app: &App<'_>, chunk: Rect)
where
B: Backend{
    let conversation_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [Constraint::Percentage(MESSAGES_HEIGHT_PERCENTAGE),                // Messages
            Constraint::Percentage(100-MESSAGES_HEIGHT_PERCENTAGE)].as_ref(),   // Input
        )
        .split(chunk);

    draw_conversation_messages(frame, app, conversation_chunks[0]);
    draw_conversation_input(frame, app, conversation_chunks[1]);
}

pub fn draw_conversation_messages<B>(frame: &mut Frame<B>, app: &App<'_>, chunk: Rect)
where
B: Backend{
    let mut title = "No Conversation Selected";
    if let Some(conversation_name) = &app.active_conversation_name{
        title = &conversation_name;
    } 

    // Get conversations
    let items: Vec<_> = app.messages_list.iter().rev()
        .map(|item|
            Spans::from(vec![
                Span::raw("["),
                Span::styled(
                    item.ts.clone(), // TODO: Format time
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("]"),
                Span::raw(" "),
                Span::raw("<"),
                Span::styled(
                    item.username.clone(),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(">"),
                Span::raw(" "),
                Span::styled(
                    item.text.clone(),
                    Style::default(),
                ),
            ])
        ).collect();

    let highlight_state = (
        app.active_block == ActiveBlock::Messages,
        app.hovered_block == ActiveBlock::Messages,
    );

    draw_paragraph(frame, app, chunk, title, items, highlight_state);
}

pub fn draw_conversation_input<B>(frame: &mut Frame<B>, app: &App<'_>, chunk: Rect)
where
B: Backend{
    let title = "Input";
    let text_input: String = app.input.iter().collect();

    let items: Vec<Spans> = vec![
        Spans::from(vec![
            Span::raw(text_input.clone()),
        ])
    ];

    let highlight_state = (
        app.active_block == ActiveBlock::Input,
        app.hovered_block == ActiveBlock::Input,
    );

    draw_paragraph(frame, app, chunk, title, items, highlight_state);
}

pub fn draw_paragraph<B>(frame: &mut Frame<B>, app: &App<'_>, chunk: Rect,
    title: &str, items: Vec<Spans>, highlight_state: (bool, bool))
where
B: Backend{
    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_type(BorderType::Plain)
        .style(Style::default().fg(Color::White))
        .border_style(util::get_color(highlight_state));

    let paragraph = Paragraph::new(items)
        .block(block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, chunk);
}

pub fn draw_selectable_list<B, S>(frame: &mut Frame<B>, app: &App<'_>, chunk: Rect,
    title: &str, items: &[S], highlight_state: (bool, bool), selected_index: &Option<usize>)
where
B: Backend,
S: std::convert::AsRef<str>{
    let mut state = ListState::default();
    state.select(*selected_index);

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
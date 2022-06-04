// use chrono::prelude::*;
use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
// use rand::{distributions::Alphanumeric, prelude::*};
use serde::{Deserialize, Serialize, de::Expected};
use std::fs;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use thiserror::Error;
use tui::{
    backend::CrosstermBackend,
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

// Input events
enum Event<T> { 
    Input(T),
    Tick,
    Quit,
    Change(T)
}

// TUI Menu structure
#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Home,
    Channels,
    Messages,
    Input,
    Search,
}

// Convert MenuItem to usize, will be used to
// highlight the current menu item using Tabs in TUI component
impl From<MenuItem> for usize {
    fn from(item: MenuItem) -> usize {
        match item {
            MenuItem::Home => 0,
            MenuItem::Channels => 1,
            MenuItem::Messages => 2,
            MenuItem::Input => 3,
            MenuItem::Search => 4,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // enable_raw_mode().expect("can run in raw mode");
    let (tx, rx) = mpsc::channel(); // Create a channel for sending and receiving events
    let tick_rate = Duration::from_millis(200); // Tick rate in milliseconds

    // Create a thread for handling input events
    thread::spawn(move || input_listen(&tx, &tick_rate));

    enable_raw_mode().expect("can run in raw mode");

    // Create a TUI component
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    
    // Create render loop
    let menu_titles = vec!["Home", "Channels", "Add", "Delete", "Quit"];
    let mut active_menu_item = MenuItem::Home;
    let mut channel_list_state = ListState::default();
    channel_list_state.select(Some(0));
    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),  // header
                        Constraint::Min(2),     // body
                        Constraint::Length(3),  // footer
                    ]
                    .as_ref(),
                )
                .split(size);

            // Create a oaragraoh widget for the footer
            let copyright = Paragraph::new("slacker-CLI 2020 - all rights reserved")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Copyright")
                        .border_type(BorderType::Plain),
                );

            // 
            let menu = menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect();

            let tabs = Tabs::new(menu)
                .select(active_menu_item.into())
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(Span::raw("|"));

            rect.render_widget(tabs, chunks[0]);
            match active_menu_item {
                MenuItem::Home => rect.render_widget(render_home(), chunks[1]),
                MenuItem::Channels => {
                    let channels_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[1]);
                    let (left, right) = render_channels(&channel_list_state);
                    rect.render_stateful_widget(left, channels_chunks[0], &mut channel_list_state);
                    rect.render_widget(right, channels_chunks[1]);
                }
                MenuItem::Input => rect.render_widget(render_home(), chunks[1]),
                MenuItem::Channels => rect.render_widget(render_home(), chunks[1]),
                _ => rect.render_widget(render_home(), chunks[1]),
            }
            rect.render_widget(copyright, chunks[2]);
        })?;

        // Receive event from input thread
        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char('h') => active_menu_item = MenuItem::Home,
                KeyCode::Char('c') => active_menu_item = MenuItem::Channels,
                KeyCode::Char('a') => {
                    
                }
                KeyCode::Char('d') => {
                    
                }
                KeyCode::Down => {

                }
                KeyCode::Up => {

                }
                _ => {}
            },
            Event::Tick => {},
            Event::Quit | Event::Change(_) => todo!(),
        }
    }


    Ok(())
}

fn input_listen(tx: &mpsc::Sender<Event<KeyEvent>>, tick_rate: &Duration) -> Result<(), io::Error> {
    let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            // wait (timeout) for user input, if no input then send tick event
            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= *tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
}

fn render_home<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "slacker-CLI",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Press 'c' to access channels.")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    home
}

fn render_channels<'a>(channel_list_state: &ListState) -> (List<'a>, Table<'a>){
    let channels = Block::default()
        .title("Channels")
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .style(Style::default().fg(Color::White));

    // TODO: Read channels from Slack API
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

    let items: Vec<_> = channel_list
        .iter()
        .map(|channel| {
            ListItem::new(Spans::from(vec![Span::styled(
                channel.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let selected_channel = channel_list
        .get(channel_list_state
            .selected()
            .expect("can get selected channel"))
        .expect("exists")
        .clone();

    let list = List::new(items).block(channels).highlight_style(
        Style::default()
            .bg(Color::LightBlue)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    // TODO: Display messages from selected channel
    let channel_detail = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_channel.name.to_string())),
        Cell::from(Span::raw(selected_channel.unread_count.to_string())),
    ])])
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "Name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Unread",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Detail")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(5),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(5),
        Constraint::Percentage(20),
    ]);

    (list, channel_detail)
}
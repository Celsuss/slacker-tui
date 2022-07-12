use tui::{
    style::{Color, Style}
};

pub fn get_color((is_active, is_hovered): (bool, bool)) -> Style {
    match (is_active, is_hovered) {
        (true, _) => Style::default().fg(Color::Cyan),
        (false, true) => Style::default().fg(Color::Magenta),
        _ => Style::default().fg(Color::White), 
    }
}
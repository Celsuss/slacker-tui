use tui::{
    style::{Color, Style}
};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

pub fn get_color((is_active, is_hovered): (bool, bool)) -> Style {
    match (is_active, is_hovered) {
        (true, _) => Style::default().fg(Color::Cyan),
        (false, true) => Style::default().fg(Color::Magenta),
        _ => Style::default().fg(Color::White), 
    }
}

pub fn calculate_character_width(character: char) -> u16 {
    UnicodeWidthChar::width(character)
        .unwrap()
        .try_into()
        .unwrap()
}
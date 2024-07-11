// External crates
extern crate ratatui;
use ratatui::prelude::*;

// Local crates
use crate::agenda_parser::parser;

pub fn create_agenda_text(agenda: parser::Agenda) -> Vec<Line<'static>> {
    let pine = Color::Rgb(62, 143, 176);
    let iris = Color::Rgb(196, 167, 231);

    let mut agenda_text: Vec<Line> = Vec::new();

    for day_event in agenda.day.iter() {
        agenda_text.push(Line::from(Span::styled(
            format!("󱃔 󰇙 {}", day_event.event),
            Style::default().fg(pine).add_modifier(Modifier::ITALIC),
        )));
    }

    for timed_event in agenda.timestamp.iter() {
        agenda_text.push(Line::from(Span::styled(
            format!(
                "{}   {} 󰇙 {}",
                timed_event.start, timed_event.end, timed_event.event
            ),
            Style::default().fg(iris).add_modifier(Modifier::ITALIC),
        )));
    }

    return agenda_text;
}

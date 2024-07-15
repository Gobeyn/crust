// External crates
extern crate ratatui;
use ratatui::prelude::*;

// Local files
use crate::configuration::config;
use crate::file;

/// Rendering of `Agenda` structures in the UI using `Config` for colors.
///
/// Given the contents of a .toml file in $HOME/.cache/crust/ and a configuration, the text
/// that should placed in the UI is returned.
pub fn render(agenda: &file::parser::Agenda, conf: &config::Config) -> Vec<Line<'static>> {
    // Initialise the text vector.
    let mut agenda_text: Vec<Line> = Vec::new();

    // Loop through the events that last the entire day.
    for day_event in agenda.day.iter() {
        // Show the event if the event is not the default, i.e. empty.
        if !day_event.event.is_empty() {
            agenda_text.push(Line::from(Span::styled(
                format!("󱃔 󰇙 {}", day_event.event),
                Style::default()
                    .fg(conf.agenda_entry_full_day_event)
                    .add_modifier(Modifier::ITALIC),
            )));
        }
    }

    // Loop through the events that are timed.
    for timed_event in agenda.timestamp.iter() {
        // Show the event if the event is not default, i.e. empty.
        if !timed_event.event.is_empty() {
            agenda_text.push(Line::from(Span::styled(
                format!(
                    "{}   {} 󰇙 {}",
                    timed_event.start, timed_event.end, timed_event.event
                ),
                Style::default()
                    .fg(conf.agenda_entry_timed_event)
                    .add_modifier(Modifier::ITALIC),
            )));
        }
    }

    return agenda_text;
}

// External crates
extern crate ratatui;
use ratatui::prelude::*;

// Local files
use crate::configuration::config;
use crate::date::date;
use crate::file;

const DAYS_SHORT: &str = "| Mo | Tu | We | Th | Fr | Sa | Su |";
const BOX: &str = "│ 󰹞  ";
const BOX_END: &str = "│ 󰹞  │";

/// Draw the calendar month.
///
/// The month drawn is the month specified by the `date` parameter. Colors of the drawn calendar
/// are determined by the `conf` argument. If a day in the shown month matches with an agenda entry
/// in $HOME/.cache/crust/, special colors are used. Similarly, if a day matches with the
/// `given_day` argument, special colors are also used.
pub fn render(
    date: &date::Date,
    given_date: &date::Date,
    conf: &config::Config,
) -> Vec<Line<'static>> {
    // Create empty vector of Lines.
    let mut calendar_text: Vec<Line> = Vec::new();
    // Add an empty line for spacing.
    calendar_text.push(Line::from(Span::raw("")));

    // Get the agenda entries in $HOME/.cache/crust/
    let entries: Vec<date::Date> = file::parser::get_agenda_entries();

    // Show the days
    calendar_text.push(Line::from(Span::styled(
        DAYS_SHORT,
        Style::default()
            .fg(conf.calendar_days_of_week)
            .bg(conf.calendar_day_bg)
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
    )));

    // Show to numbered days, or blocks if that day does not belong in the shown month.
    // Initialise some variables
    let first_of_month: date::Date = date::Date {
        day: 1,
        month: date.month,
        year: date.year,
    };
    let first_weekday_of_month = first_of_month.day_of_week();
    let days_in_month = first_of_month.max_day();

    let mut days_line: Vec<Span> = Vec::new();
    let mut weekday_counter = first_weekday_of_month;

    // Draw boxes for the days in the week before the first day of the month.
    if first_weekday_of_month > 1 {
        for _ in 1..first_weekday_of_month {
            days_line.push(Span::styled(
                BOX,
                Style::default()
                    .fg(conf.calendar_day)
                    .bg(conf.calendar_day_bg)
                    .add_modifier(Modifier::BOLD),
            ));
        }
    }

    // Loop through all days in the month.
    for day_counter in 1..=days_in_month {
        let current_date = date::Date {
            day: day_counter,
            ..first_of_month
        };

        let style: Style = {
            if current_date == *given_date {
                Style::default()
                    .fg(conf.calendar_day_selected)
                    .bg(conf.calendar_day_selected_bg)
                    .add_modifier(Modifier::BOLD | Modifier::ITALIC)
            } else if entries.contains(&current_date) {
                Style::default()
                    .fg(conf.calendar_day_with_entry)
                    .bg(conf.calendar_day_with_entry_bg)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(conf.calendar_day)
                    .bg(conf.calendar_day_bg)
                    .add_modifier(Modifier::BOLD)
            }
        };

        if day_counter < 10 {
            days_line.push(Span::styled(format!("│ {}  ", day_counter), style));
        } else {
            days_line.push(Span::styled(format!("│ {} ", day_counter), style));
        }

        weekday_counter += 1;
        if weekday_counter > 7 {
            days_line.push(Span::styled(
                "│",
                Style::new()
                    .fg(conf.calendar_day)
                    .bg(conf.calendar_day_bg)
                    .add_modifier(Modifier::BOLD),
            ));
            weekday_counter = 1;
            calendar_text.push(Line::from(days_line));
            days_line = Vec::new();
        }
    }

    if !days_line.is_empty() {
        while weekday_counter < 7 {
            days_line.push(Span::styled(
                BOX,
                Style::default()
                    .fg(conf.calendar_day)
                    .bg(conf.calendar_day_bg)
                    .add_modifier(Modifier::BOLD),
            ));
            weekday_counter += 1;
        }
        days_line.push(Span::styled(
            BOX_END,
            Style::default()
                .fg(conf.calendar_day)
                .bg(conf.calendar_day_bg)
                .add_modifier(Modifier::BOLD),
        ));
        calendar_text.push(Line::from(days_line));
    }

    return calendar_text;
}

// External crates
extern crate ratatui;

use ratatui::prelude::*;

// Local files
use crate::agenda_parser::entry_search;
use crate::calendar_logic::date_conversions;

pub fn create_calendar_text(
    month: i32,
    year: i32,
    current_date: &entry_search::Date,
) -> Vec<Line<'static>> {
    let rose = Color::Rgb(234, 154, 151);
    let gold = Color::Rgb(246, 193, 119);
    let darkened_gold = Color::Rgb(151, 92, 10);
    let overlay = Color::Rgb(57, 53, 82);
    let muted = Color::Rgb(110, 106, 134);
    let iris = Color::Rgb(196, 167, 231);

    let entries: Vec<entry_search::Date> = entry_search::get_agenda_entries();
    //entries.sort();

    let mut calendar_text: Vec<Line> = Vec::new();
    calendar_text.push(Line::from(Span::raw(" ")));

    // Show the days
    calendar_text.push(Line::from(Span::styled(
        "| Mo | Tu | We | Th | Fr | Sa | Su |",
        Style::default()
            .fg(rose)
            .bg(muted)
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
    )));

    // Show number day
    let first_day_of_the_month = date_conversions::date_to_day_of_the_week(1, month, year);
    let days_in_month = date_conversions::month_to_days_in_month(month, year);
    let mut days_line: Vec<Span> = Vec::new();
    let mut weekday_counter = first_day_of_the_month;

    // 󰹞, 
    if first_day_of_the_month > 1 {
        for _ in 1..first_day_of_the_month {
            days_line.push(Span::styled(
                format!("│ 󰹞  "),
                Style::default()
                    .fg(gold)
                    .bg(overlay)
                    .add_modifier(Modifier::BOLD),
            ));
        }
    }

    for day_counter in 1..=days_in_month {
        let date = entry_search::Date {
            day: day_counter,
            month, // Shorthand for month: month
            year,
        };

        let style: Style = {
            if date == *current_date {
                Style::default()
                    .fg(darkened_gold)
                    .bg(rose)
                    .add_modifier(Modifier::BOLD | Modifier::ITALIC)
            } else if entries.contains(&date) {
                Style::default()
                    .fg(darkened_gold)
                    .bg(iris)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(gold)
                    .bg(overlay)
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
                    .fg(gold)
                    .bg(overlay)
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
                format!("│ 󰹞  "),
                Style::default()
                    .fg(gold)
                    .bg(overlay)
                    .add_modifier(Modifier::BOLD),
            ));
            weekday_counter += 1;
        }
        days_line.push(Span::styled(
            format!("│ 󰹞  │"),
            Style::default()
                .fg(gold)
                .bg(overlay)
                .add_modifier(Modifier::BOLD),
        ));
        calendar_text.push(Line::from(days_line));
    }

    return calendar_text;
}

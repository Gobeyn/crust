// External crates
extern crate ratatui;

use ratatui::prelude::*;

// Local files
use crate::calendar_logic::date_conversions;

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

// BUG: The month January 2006 does not return the correct result. January 1st is a
// Sunday and this program shows it on Monday. Similarly for other January months.
// Perhaps an error occurs in one of the date conversion functions when using January 1st.

pub fn create_calendar_text(month: i32, year: i32) -> Vec<Line<'static>> {
    let love = Color::Rgb(235, 111, 146);
    let rose = Color::Rgb(234, 154, 151);
    let gold = Color::Rgb(246, 193, 119);
    let overlay = Color::Rgb(57, 53, 82);
    let muted = Color::Rgb(110, 106, 134);

    let mut calendar_text: Vec<Line> = Vec::new();

    // Show the month and year
    let new_month = month - 1;
    let month_string: String = {
        if new_month as usize <= MONTHS.len() {
            MONTHS[new_month as usize].to_string()
        } else {
            panic!("'months' input was out of bounds.");
        }
    };
    calendar_text.push(Line::from(Span::raw("")));
    calendar_text.push(Line::from(Span::styled(
        format!("{} {}", month_string, year),
        Style::default()
            .fg(love)
            .add_modifier(Modifier::BOLD | Modifier::ITALIC),
    )));
    calendar_text.push(Line::from(Span::raw("")));

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
        if day_counter < 10 {
            days_line.push(Span::styled(
                format!("│ {}  ", day_counter),
                Style::default()
                    .fg(gold)
                    .bg(overlay)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            days_line.push(Span::styled(
                format!("│ {} ", day_counter),
                Style::default()
                    .fg(gold)
                    .bg(overlay)
                    .add_modifier(Modifier::BOLD),
            ));
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

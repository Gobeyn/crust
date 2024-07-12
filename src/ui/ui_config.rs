extern crate chrono;
extern crate ratatui;

use ratatui::{prelude::*, widgets::*};

use super::agenda_render;
use super::calendar_render;
use crate::agenda_parser::entry_search;
use crate::agenda_parser::parser;
use crate::argument_handling::handler;
use crate::calendar_logic::date_conversions;

#[derive(Debug)]
struct MonthAndYear {
    month: i32,
    year: i32,
}

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

const DAYS: [&str; 7] = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];

fn get_next_month_and_year(month_and_year: &MonthAndYear) -> MonthAndYear {
    let mut next_month = month_and_year.month + 1;
    let mut next_year = month_and_year.year;

    if next_month > 12 {
        next_month = 1;
        next_year += 1;
    }

    let next_month_and_year = MonthAndYear {
        month: next_month,
        year: next_year,
    };

    return next_month_and_year;
}

fn get_month_string(month: i32) -> String {
    let month_string: String = {
        let month_shift: i32 = month - 1;
        if month_shift as usize <= MONTHS.len() {
            MONTHS[month_shift as usize].to_string()
        } else {
            panic!("Month index was out of range.");
        }
    };
    return month_string;
}

fn get_month_year_string(month_and_year: &MonthAndYear) -> String {
    let month_string = get_month_string(month_and_year.month);
    let month_year_string = format!(" {} {} ", month_string, month_and_year.year);
    return month_year_string;
}

fn get_day_string(day_of_week: i32) -> String {
    let day_string: String = {
        let day_shift = day_of_week - 1;
        if day_shift as usize <= DAYS.len() {
            DAYS[day_shift as usize].to_string()
        } else {
            panic!("Day index was out of range.");
        }
    };
    return day_string;
}

fn get_agenda_entry_title(day: i32, month: i32, year: i32) -> String {
    let day_of_week = date_conversions::date_to_day_of_the_week(day, month, year);
    let day_string = get_day_string(day_of_week);
    let month_string = get_month_string(month);
    let day_end: &str = {
        if day == 1 || day == 21 || day == 31 {
            "st"
        } else if day == 2 || day == 22 {
            "nd"
        } else {
            "th"
        }
    };
    let agenda_title: String = format!(
        "  Agenda 󰇙 {}, {} {}{} {} ",
        day_string, month_string, day, day_end, year
    );
    return agenda_title;
}

pub fn ui_crust_higher_order(program_args: handler::ProgramArguments) -> Box<dyn Fn(&mut Frame)> {
    // Return UI function with program argument values filled in.
    Box::new(move |frame: &mut Frame| {
        // Define colors
        let foam = Color::Rgb(156, 207, 216);
        let pine = Color::Rgb(62, 143, 116);
        let love = Color::Rgb(235, 111, 146);

        // Define outer layout
        let layout = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(40), Constraint::Percentage(60)],
        )
        .split(frame.size());

        // Calendar UI
        let calendar_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ],
        )
        .margin(2)
        .split(layout[0]);

        // Define outer Calendar block
        let calendar_block = Block::new()
            .title("   Calendar ")
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(pine)
                    .add_modifier(Modifier::BOLD | Modifier::ITALIC),
            )
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(foam));

        // Define three subcalendar blocks for the asked for month and two months ahead
        let current_date = entry_search::Date {
            day: program_args.day,
            month: program_args.month,
            year: program_args.year,
        };
        let current_month_and_year = MonthAndYear {
            month: program_args.month,
            year: program_args.year,
        };
        let next_month_and_year = get_next_month_and_year(&current_month_and_year);
        let second_next_month_and_year = get_next_month_and_year(&next_month_and_year);

        let current_month_year_str = get_month_year_string(&current_month_and_year);
        let next_month_year_str = get_month_year_string(&next_month_and_year);
        let second_next_month_year_str = get_month_year_string(&second_next_month_and_year);

        let current_month_block = Block::new()
            .title(current_month_year_str)
            .title_alignment(Alignment::Center)
            .title_style(Style::default().fg(love).add_modifier(Modifier::BOLD))
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(pine));
        let next_month_block = Block::new()
            .title(next_month_year_str)
            .title_alignment(Alignment::Center)
            .title_style(Style::default().fg(love).add_modifier(Modifier::BOLD))
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(pine));
        let second_next_month_block = Block::new()
            .title(second_next_month_year_str)
            .title_alignment(Alignment::Center)
            .title_style(Style::default().fg(love).add_modifier(Modifier::BOLD))
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(pine));

        // Define the text to be put into these calendar blocks
        let current_month_text = calendar_render::create_calendar_text(
            current_month_and_year.month,
            current_month_and_year.year,
            &current_date,
        );
        let next_month_text = calendar_render::create_calendar_text(
            next_month_and_year.month,
            next_month_and_year.year,
            &current_date,
        );
        let second_next_month_text = calendar_render::create_calendar_text(
            second_next_month_and_year.month,
            second_next_month_and_year.year,
            &current_date,
        );

        // Define the paragraph objects for each of the calendar blocks

        let current_month_paragraph = Paragraph::new(current_month_text)
            .block(current_month_block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        let next_month_paragraph = Paragraph::new(next_month_text)
            .block(next_month_block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        let second_next_month_paragraph = Paragraph::new(second_next_month_text)
            .block(second_next_month_block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        // Render onto the frame
        frame.render_widget(calendar_block, layout[0]);
        frame.render_widget(current_month_paragraph, calendar_layout[0]);
        frame.render_widget(next_month_paragraph, calendar_layout[1]);
        frame.render_widget(second_next_month_paragraph, calendar_layout[2]);

        // Preview UI
        let entries: Vec<entry_search::Date> = entry_search::get_agenda_entries();
        let mut filtered_entries: Vec<entry_search::Date> = entries
            .into_iter()
            .filter(|entry| *entry >= current_date)
            .collect();
        filtered_entries.sort();

        let next_nonempty: entry_search::Date = {
            if filtered_entries.len() >= 2 {
                if filtered_entries[0] == current_date {
                    filtered_entries[1].clone()
                } else {
                    filtered_entries[0].clone()
                }
            } else {
                current_date
            }
        };

        let preview_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .margin(2)
        .split(layout[1]);

        // Define outer preview block
        let preview_block = Block::new()
            .title("   Agenda ")
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(pine)
                    .add_modifier(Modifier::BOLD | Modifier::ITALIC),
            )
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(foam));

        // Define blocks for submodules in preview
        let current_preview_title =
            get_agenda_entry_title(program_args.day, program_args.month, program_args.year);
        let current_preview_block = Block::new()
            .title(current_preview_title)
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(love)
                    .add_modifier(Modifier::BOLD | Modifier::ITALIC),
            )
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(pine));

        let next_preview_title =
            get_agenda_entry_title(next_nonempty.day, next_nonempty.month, next_nonempty.year);
        let next_preview_block = Block::new()
            .title(next_preview_title)
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(love)
                    .add_modifier(Modifier::BOLD | Modifier::ITALIC),
            )
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(pine));

        // Define text to show inside the boxes
        let mut current_preview_filedir: std::path::PathBuf =
            parser::date_to_filedir(program_args.day, program_args.month, program_args.year);

        let current_preview_text = {
            if std::path::Path::exists(&current_preview_filedir) {
                let current_preview_agenda: parser::Agenda =
                    parser::parse_agenda_toml(&mut current_preview_filedir);
                agenda_render::create_agenda_text(current_preview_agenda)
            } else {
                let mut agenda_text: Vec<Line> = Vec::new();
                agenda_text.push(Line::from(Span::styled(
                    "No entry for this date.",
                    Style::default().fg(love).add_modifier(Modifier::BOLD),
                )));
                agenda_text
            }
        };

        let mut next_preview_filedir: std::path::PathBuf =
            parser::date_to_filedir(next_nonempty.day, next_nonempty.month, next_nonempty.year);

        let next_preview_text = {
            if std::path::Path::exists(&next_preview_filedir) {
                let next_preview_agenda: parser::Agenda =
                    parser::parse_agenda_toml(&mut next_preview_filedir);
                agenda_render::create_agenda_text(next_preview_agenda)
            } else {
                let mut agenda_text: Vec<Line> = Vec::new();
                agenda_text.push(Line::from(Span::styled(
                    "No entry for this date.",
                    Style::default().fg(love).add_modifier(Modifier::BOLD),
                )));
                agenda_text
            }
        };

        // Define paragraphs for preview boxes
        let current_preview_paragraph = Paragraph::new(current_preview_text)
            .block(current_preview_block)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        let next_preview_paragraph = Paragraph::new(next_preview_text)
            .block(next_preview_block)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        // Render onto frame
        frame.render_widget(preview_block, layout[1]);
        frame.render_widget(current_preview_paragraph, preview_layout[0]);
        frame.render_widget(next_preview_paragraph, preview_layout[1]);
    })
}

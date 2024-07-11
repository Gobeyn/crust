extern crate chrono;
extern crate ratatui;

use ratatui::{prelude::*, widgets::*};

use super::agenda_render;
use super::calendar_render;
use crate::agenda_parser::parser;
use crate::argument_handling::handler;

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
        );
        let next_month_text = calendar_render::create_calendar_text(
            next_month_and_year.month,
            next_month_and_year.year,
        );
        let second_next_month_text = calendar_render::create_calendar_text(
            second_next_month_and_year.month,
            second_next_month_and_year.year,
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
        let current_preview_block = Block::new()
            .title(format!(
                " Agenda 󰇙 {}-{}-{} ",
                program_args.day, program_args.month, program_args.year
            ))
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(love)
                    .add_modifier(Modifier::BOLD | Modifier::ITALIC),
            )
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(pine));

        let next_preview_block = Block::new()
            .title(" Agenda [NEXT NON-EMPTY] ")
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

        // Define paragraphs for preview boxes
        let current_preview_paragraph = Paragraph::new(current_preview_text)
            .block(current_preview_block)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .scroll((0, 0));

        // Render onto frame
        frame.render_widget(preview_block, layout[1]);
        frame.render_widget(current_preview_paragraph, preview_layout[0]);
        frame.render_widget(next_preview_block, preview_layout[1]);
    })
}

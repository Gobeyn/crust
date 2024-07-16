// External crates
extern crate ratatui;
use ratatui::{prelude::*, widgets::*};

// Local files
use super::{agenda, calendar};
use crate::args;
use crate::configuration::config;
use crate::date::date;
use crate::file;

// TODO: Read up on lifetimes so we can use references to `ProgramArguments` and `Config` instead
// of taking ownership and needing to copy it multiple times in the window.rs and main.rs files.
// TODO: Add an error UI like `btop` when the UI is too small for the default and the restricted
// UI.

/// Create UI with `ProgramArguments` filled in as arguments.
///
/// Function returns a function on the `Frame` as required by `Ratatui`.
pub fn ui_pre_args(
    program_args: args::parser::ProgramArguments,
    conf: config::Config,
) -> Box<dyn Fn(&mut Frame)> {
    Box::new(move |frame: &mut Frame| {
        // Define outer layout
        let layout = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(40), Constraint::Percentage(60)],
        )
        .split(frame.size());

        // ========== Calendar UI ==========
        // Calendar layout
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

        // Define outer calendar block
        let calendar_block = Block::new()
            .title("   Calendar ")
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(conf.calendar_title)
                    .add_modifier(Modifier::BOLD | Modifier::ITALIC),
            )
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(conf.calendar_box));

        // Get the next two months with respect to the Date passed in program_args, and the
        // corresponding month strings for those months.
        let next_month: date::Date = program_args.date.first_of_next_month();
        let second_next_month: date::Date = next_month.first_of_next_month();

        let given_month_title = program_args.date.calendar_title();
        let next_month_title = next_month.calendar_title();
        let second_next_month_title = second_next_month.calendar_title();

        // Define the three calendar sub-blocks with the given month and the next two months.

        let given_month_block = Block::new()
            .title(given_month_title)
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(conf.calendar_month_title)
                    .add_modifier(Modifier::BOLD),
            )
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(conf.calendar_month_box));

        let next_month_block = Block::new()
            .title(next_month_title)
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(conf.calendar_month_title)
                    .add_modifier(Modifier::BOLD),
            )
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(conf.calendar_month_box));

        let second_next_month_block = Block::new()
            .title(second_next_month_title)
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(conf.calendar_month_title)
                    .add_modifier(Modifier::BOLD),
            )
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(conf.calendar_month_box));

        // Define text to be put into the calendar blocks.
        let given_month_text = calendar::render(&program_args.date, &program_args.date, &conf);
        let next_month_text = calendar::render(&next_month, &program_args.date, &conf);
        let second_next_month_text =
            calendar::render(&second_next_month, &program_args.date, &conf);

        // Define Paragraph objects onto the Blocks.
        let given_month_par = Paragraph::new(given_month_text)
            .block(given_month_block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        let next_month_par = Paragraph::new(next_month_text)
            .block(next_month_block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        let second_next_month_par = Paragraph::new(second_next_month_text)
            .block(second_next_month_block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        // Render onto the frame.
        frame.render_widget(calendar_block, layout[0]);
        frame.render_widget(given_month_par, calendar_layout[0]);
        frame.render_widget(next_month_par, calendar_layout[1]);
        frame.render_widget(second_next_month_par, calendar_layout[2]);

        // ========== Agenda UI ==========
        // Get the entries.
        let entries: Vec<date::Date> = file::parser::get_agenda_entries();
        // Filter and sort to dates equal or older than the given date.
        let mut filtered_entries: Vec<date::Date> = entries
            .into_iter()
            .filter(|entry| *entry >= program_args.date)
            .collect();
        filtered_entries.sort();
        // Extract the next non-empty agenda entry.
        let next_nonempty: date::Date = {
            if filtered_entries.len() >= 2 {
                if filtered_entries[0] == program_args.date {
                    filtered_entries[1].clone()
                } else {
                    filtered_entries[0].clone()
                }
            } else {
                program_args.date
            }
        };

        // Define the Agenda layout.
        let agenda_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .margin(2)
        .split(layout[1]);

        // Define agenda block.
        let agenda_block = Block::new()
            .title("   Agenda ")
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(conf.agenda_title)
                    .add_modifier(Modifier::BOLD | Modifier::ITALIC),
            )
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(conf.agenda_box));

        // Define agenda sub-blocks.
        let given_agenda_title = program_args.date.agenda_title();
        let given_agenda_block = Block::new()
            .title(given_agenda_title)
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(conf.agenda_entry_title)
                    .add_modifier(Modifier::BOLD | Modifier::ITALIC),
            )
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(conf.agenda_entry_box));

        let next_agenda_title = next_nonempty.agenda_title();
        let next_agenda_block = Block::new()
            .title(next_agenda_title)
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(conf.agenda_entry_title)
                    .add_modifier(Modifier::BOLD | Modifier::ITALIC),
            )
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(conf.agenda_entry_box));

        // Obtain the agenda text from the two dates.
        let given_agenda_text = match program_args.date.get_agenda() {
            Some(given_agenda) => agenda::render(&given_agenda, &conf),
            None => {
                let agenda_text: Vec<Line> = vec![Line::from(Span::styled(
                    "No entry for this date.",
                    Style::default().fg(conf.agenda_entry_full_day_event),
                ))];
                agenda_text
            }
        };
        let next_agenda_text = match next_nonempty.get_agenda() {
            Some(next_agenda) => agenda::render(&next_agenda, &conf),
            None => {
                let agenda_text: Vec<Line> = vec![Line::from(Span::styled(
                    "No entry for this date.",
                    Style::default().fg(conf.agenda_entry_full_day_event),
                ))];
                agenda_text
            }
        };

        // Define `Paragraphs` for the agenda content.
        let given_agenda_par = Paragraph::new(given_agenda_text)
            .block(given_agenda_block)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        let next_agenda_par = Paragraph::new(next_agenda_text)
            .block(next_agenda_block)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        // Render onto the frame.
        frame.render_widget(agenda_block, layout[1]);
        frame.render_widget(given_agenda_par, agenda_layout[0]);
        frame.render_widget(next_agenda_par, agenda_layout[1]);
    })
}

/// Same as `ui_pre_args` but for a restricted, vertical half-screen, layout.
pub fn ui_restricted_vertical_pre_args(
    program_args: args::parser::ProgramArguments,
    conf: config::Config,
) -> Box<dyn Fn(&mut Frame)> {
    Box::new(move |frame: &mut Frame| {
        // Define layout
        let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(33), Constraint::Percentage(66)],
        )
        .split(frame.size());

        // ========== Calendar UI ==========
        let cal_title = program_args.date.calendar_title();
        let cal_block = Block::new()
            .title(cal_title)
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(conf.calendar_month_title)
                    .add_modifier(Modifier::BOLD),
            )
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(conf.calendar_month_box));
        let cal_text = calendar::render(&program_args.date, &program_args.date, &conf);
        let cal_par = Paragraph::new(cal_text)
            .block(cal_block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        frame.render_widget(cal_par, layout[0]);

        // ========== Agenda UI ==========
        let agenda_title = program_args.date.agenda_title();
        let agenda_block = Block::new()
            .title(agenda_title)
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(conf.agenda_entry_title)
                    .add_modifier(Modifier::BOLD | Modifier::ITALIC),
            )
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(conf.agenda_entry_box));
        let agenda_text = match program_args.date.get_agenda() {
            Some(ag) => agenda::render(&ag, &conf),
            None => {
                let txt: Vec<Line> = vec![Line::from(Span::styled(
                    "No entry for this date.",
                    Style::default().fg(conf.agenda_entry_full_day_event),
                ))];
                txt
            }
        };
        let agenda_par = Paragraph::new(agenda_text)
            .block(agenda_block)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        frame.render_widget(agenda_par, layout[1]);
    })
}

// External crates
extern crate crossterm;
extern crate getopts;
extern crate ratatui;

// Local files
use crust::agenda_entry::entry;
use crust::agenda_parser::entry_search;
use crust::agenda_parser::parser;
use crust::argument_handling::handler;
use crust::ui::{event_handler, ui_config};

fn main() {
    // Get program arguments or their default values.
    let program_args: handler::ProgramArguments = handler::parse_arguments();
    let program_args_for_ui: handler::ProgramArguments = program_args.clone();

    if program_args.open_calendar {
        let _ = open_ui(program_args_for_ui);
    } else {
        if program_args.edit {
            open_editor(&program_args);
        } else if program_args.remove {
            remove_file(&program_args);
        } else {
            entry::handle_agenda_entry(&program_args);
        }
    }
}

fn remove_file(program_args: &handler::ProgramArguments) {
    let filepath = parser::date_to_filedir(program_args.day, program_args.month, program_args.year);
    if filepath.exists() {
        std::fs::remove_file(filepath).unwrap();
    } else {
        println!("Agenda entry for this date does not exist.");
    }
}

fn open_editor(program_args: &handler::ProgramArguments) {
    let filepath = parser::date_to_filedir(program_args.day, program_args.month, program_args.year);
    let editor = std::env::var("EDITOR").expect("Could not find $EDITOR system variable.");
    std::process::Command::new(editor)
        .arg(&filepath)
        .status()
        .expect(
            "Unable to open file with
        $EDITOR.",
        );
}

fn open_ui(program_args: handler::ProgramArguments) -> std::io::Result<()> {
    let program_args_copy = program_args.clone();
    let mut ui = ui_config::ui_crust_higher_order(program_args);
    // Enable raw mode, this disables typical inputs, typed text is not shown on the
    // screen, is not processed when pressing enter, new line character is not processed,
    // etc. Importantly, println! should be replaced by write!. For more information see
    // docs: https://docs.rs/crossterm/latest/crossterm/terminal/index.html#raw-mode
    crossterm::terminal::enable_raw_mode()?;

    // Switch to alternate screen that lives on top of the terminal. For this we
    // execute the EnterAlternateScreen command from the crossterm crate
    // docs: https://docs.rs/crossterm/latest/crossterm/terminal/struct.EnterAlternateScreen.html
    crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen)?;

    // In the alternate screen, we open a new terminal from Ratatui, on which we can use
    // the functions provided by it.
    // docs: https://docs.rs/ratatui/latest/ratatui/struct.Terminal.html
    // In the function we interface it with a terminal library,
    // for this we will use Crossterm.
    // docs: https://docs.rs/ratatui/latest/ratatui/backend/struct.CrosstermBackend.html
    // This is the reason we needed to call the previous two functions.
    let mut terminal =
        ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(std::io::stdout()))?;

    // Enter a UI drawing loop with event handling so we can exit the program
    let mut should_quit = false;
    let mut day_shift_counter = 0;
    while !should_quit {
        let _ = terminal.draw(&ui);
        let key_events = event_handler::get_key_events()?;
        should_quit = key_events.quit;

        if key_events.next_entry {
            day_shift_counter += 1;
            let mut program_args_ui = program_args_copy.clone();
            let mut new_date: entry_search::Date = entry_search::Date {
                day: program_args_ui.day,
                month: program_args_ui.month,
                year: program_args_ui.year,
            };
            entry_search::add_days_to_date(&mut new_date, day_shift_counter);
            program_args_ui.day = new_date.day;
            program_args_ui.month = new_date.month;
            program_args_ui.year = new_date.year;
            ui = ui_config::ui_crust_higher_order(program_args_ui);
        } else if key_events.prev_entry {
            day_shift_counter -= 1;
            let mut program_args_ui = program_args_copy.clone();
            let mut new_date: entry_search::Date = entry_search::Date {
                day: program_args_ui.day,
                month: program_args_ui.month,
                year: program_args_ui.year,
            };
            entry_search::add_days_to_date(&mut new_date, day_shift_counter);
            program_args_ui.day = new_date.day;
            program_args_ui.month = new_date.month;
            program_args_ui.year = new_date.year;
            ui = ui_config::ui_crust_higher_order(program_args_ui);
        }
    }

    // Disable raw mode so we can return to normal terminal function.
    crossterm::terminal::disable_raw_mode()?;

    // Leave the alternate screen and return to the original terminal.
    crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;

    // Return Ok if we get to the end of the function.
    Ok(())
}

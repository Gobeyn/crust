// External crates
extern crate crossterm;
extern crate ratatui;

// Local files
use super::ui;
use crate::args;
use crate::configuration::config;
use crate::key::event;

/// Create alternate window for UI.
///
/// Enter an alternate window on top of the current terminal session using `crossterm` and
/// draw a UI in it using `Ratatui`. The possible key events are listed in the `KeyEvents` enum.
pub fn create_window(program_args: args::parser::ProgramArguments, conf: config::Config) {
    let prog_args_copy = program_args.clone();
    let conf_copy = conf.clone();
    let mut user_interface = ui::ui_pre_args(program_args, conf);

    // Enable raw mode, this disables typical user input like typing.
    let _ = match crossterm::terminal::enable_raw_mode() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error enabling raw mode: {e}");
            return;
        }
    };

    // Switch to alternate screen on top of the terminal.
    let _ = match crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen)
    {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error changing to alternate screen: {e}");
            return;
        }
    };

    // In alternate screen open a new terminal from Ratatui interfaced with crossterm.
    let mut terminal =
        match ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(std::io::stdout())) {
            Ok(term) => term,
            Err(e) => {
                eprintln!("Error creating Ratatui terminal: {e}");
                return;
            }
        };

    // Enter UI drawing loop
    let mut run = true;
    let mut day_shift_counter = 0;
    while run {
        let _ = terminal.draw(&user_interface);
        let key_event: event::KeyEvents = event::get_key_event();
        match key_event {
            event::KeyEvents::Quit => {
                run = false;
            }
            event::KeyEvents::Next => {
                // Update date shift.
                day_shift_counter += 1;
                // Copy the program arguments and the configuration.
                let mut prog_args_ui = prog_args_copy.clone();
                let conf_ui = conf_copy.clone();
                // Update the `date` stored in the program arguments.
                prog_args_ui.date.add_days(day_shift_counter);
                // Update the UI.
                user_interface = ui::ui_pre_args(prog_args_ui, conf_ui);
            }
            event::KeyEvents::Previous => {
                // Update date shift.
                day_shift_counter -= 1;
                // Copy the program arguments and configuration.
                let mut prog_args_ui = prog_args_copy.clone();
                let conf_ui = conf_copy.clone();
                // Update the `date` stored in the program arguments.
                prog_args_ui.date.add_days(day_shift_counter);
                // Update the UI.
                user_interface = ui::ui_pre_args(prog_args_ui, conf_ui);
            }
            _ => {}
        }
    }

    // Disable raw mode so we return to normal terminal function.
    let _ = match crossterm::terminal::disable_raw_mode() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error disabling raw mode: {e}");
            return;
        }
    };

    // Leave the alternate screen and return to original terminal.
    let _ = match crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)
    {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error leaving alternate screen: {e}");
            return;
        }
    };
}

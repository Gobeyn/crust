// External crates
extern crate crossterm;
extern crate getopts;
extern crate ratatui;

// Local files
use crust::argument_handling::handler;
use crust::ui::{event_handler, ui_config};

fn main() -> std::io::Result<()> {
    // Get the arguments for the program and their default values.
    let program_args: handler::ProgramArguments = handler::parse_arguments();
    let ui = ui_config::ui_crust_higher_order(program_args);

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
    while !should_quit {
        //terminal.draw(ui_config::ui_crust)?;
        let _ = terminal.draw(&ui);
        should_quit = event_handler::handle_events()?;
    }

    // Disable raw mode so we can return to normal terminal function.
    crossterm::terminal::disable_raw_mode()?;

    // Leave the alternate screen and return to the original terminal.
    crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;

    // Return Ok if we get to the end of the function.
    Ok(())
}

// Local files
use crust::args;
use crust::configuration::config;
use crust::file;
use crust::ui::window;

fn main() {
    let prog_args = args::parser::parse_arguments();
    let prog_args_ui = prog_args.clone();
    let conf = config::Config::default();

    if prog_args.flags.open_calendar {
        let _ = window::create_window(prog_args_ui, conf);
    } else {
        if prog_args.flags.edit {
            file::parser::open_editor(&prog_args);
        } else if prog_args.flags.remove {
            file::parser::remove_file(&prog_args);
        } else {
            file::parser::write_entry(&prog_args);
        }
    }
}

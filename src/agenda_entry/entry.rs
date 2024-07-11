use crate::agenda_parser::parser;
use crate::argument_handling::handler;

use std::io::Write;

pub fn handle_agenda_entry(program_args: &handler::ProgramArguments) {
    // Get file directory in the correct format using provided program arguments.
    let filedir: std::path::PathBuf =
        parser::date_to_filedir(program_args.day, program_args.month, program_args.year);

    // Open file if it exists and create it if it does not.
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(filedir)
        .unwrap();

    // Check if the full_day flag was set.
    if program_args.full_day {
        // Write the entry under [[day]] in the .toml file
        file.write("[[day]]\n".as_bytes())
            .expect("Error when writing to file.");
        file.write(format!("event = '{}'\n", program_args.entry).as_bytes())
            .expect("Error when writing to file.");
    } else {
        // Otherwise, we assume a timestampe was given and the entry falls under [[timestamp]]
        // in the .toml file.
        file.write("[[timestamp]]\n".as_bytes())
            .expect("Error when writing to file.");
        file.write(format!("start = '{}'\n", program_args.start).as_bytes())
            .expect("Error when writing to file.");
        file.write(format!("end = '{}'\n", program_args.end).as_bytes())
            .expect("Error when writing to file.");
        file.write(format!("event = '{}'\n", program_args.entry).as_bytes())
            .expect("Error when writing to file.");
    }
}

// External crates
extern crate dirs;
extern crate regex;
extern crate serde;
extern crate toml;

use serde::{Deserialize, Serialize};
use std::io::Read;
use std::io::Write;

// Local files
use crate::args;
use crate::configuration::config;
use crate::date::date;

/// Store full day events.
#[derive(Deserialize, Serialize, Debug)]
pub struct DayEvent {
    pub event: String,
}

/// Store events between two time stamps.
///
/// Time stamps `start` and `end` have the assumed form xy:zw.
#[derive(Deserialize, Serialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TimedEvent {
    pub start: String,
    pub end: String,
    pub event: String,
}

/// Deserialization struct for the agenda .toml files
///
/// Full day events are use the `DayEvent` struct and timed events are contained in
/// the `TimedEvent` struct. If a .toml file cannot be entirely deserialized into the struct,
/// the remaining fields are filled in with default values.
#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
pub struct Agenda {
    pub day: Vec<DayEvent>,
    pub timestamp: Vec<TimedEvent>,
}

impl Default for Agenda {
    /// Default values for `Agenda` are vectors with one element and empty strings.
    fn default() -> Self {
        Agenda {
            day: vec![DayEvent {
                event: "".to_string(),
            }],
            timestamp: vec![TimedEvent {
                start: "".to_string(),
                end: "".to_string(),
                event: "".to_string(),
            }],
        }
    }
}

/// Convert agenda information to `Agenda` struct.
///
/// Given the filepath `$HOME/.cache/crust/dd-mm-yyyy.toml`, parse the file into the `Agenda`
/// struct. Any field that cannot be filled in by the `.toml` file is filled in by the `Default`
/// implementation on `Agenda`.
pub fn parse_agenda_toml(filedir: &mut std::path::PathBuf) -> Option<Agenda> {
    // Open the file, return None if an error occurs.
    let mut file = match std::fs::File::open(filedir) {
        Ok(v) => v,
        Err(_) => {
            return None;
        }
    };
    let mut contents = String::new();
    // Read the contents of the file into a String, return None if an error occurs.
    let _ = match file.read_to_string(&mut contents) {
        Ok(v) => v,
        Err(_) => {
            return None;
        }
    };

    // Parse the file contents into a `toml::Value`, return None if an error occurs.
    let parsed: toml::Value = match toml::from_str(&contents) {
        Ok(v) => v,
        Err(_) => {
            return None;
        }
    };

    // Parse the `toml::Value` into the custom `Agenda` struct, filling in blank fields with
    // default values and return None if an error occurs.
    let mut toml_struct: Agenda = match parsed.try_into() {
        Ok(v) => v,
        Err(_) => {
            return None;
        }
    };
    toml_struct.timestamp.sort();
    return Some(toml_struct);
}

/// Get a vector of dates stored in $HOME/.cache/crust/.
///
/// If any error occurs when attempting to find the $HOME/.cache/crust/ directory, or listing the
/// files in that directory, we return an empty vector. Only files of the form `dd-mm-yyyy.toml`
/// are converted into a `Date` structure, and only the `Date` structures validated with the
/// `validate` method on `Date` are pushed onto the vector of valid entries.
pub fn get_agenda_entries() -> Vec<date::Date> {
    // Initialise the output vector.
    let empty: Vec<date::Date> = Vec::new();
    let mut valid_entries: Vec<date::Date> = Vec::new();

    // Get the $HOME/.cache/crust path
    let mut filedir: std::path::PathBuf = match dirs::cache_dir() {
        Some(v) => v,
        None => {
            return empty;
        }
    };
    filedir.push("crust");

    // Get a list of files in that directory
    let paths = match std::fs::read_dir(filedir) {
        Ok(v) => v,
        Err(_) => {
            return empty;
        }
    };

    // Define a regular expression to extract the correct file name structure with capture groups for
    // the day, month and year.
    let re = match regex::Regex::new(
        r"^([0-3][0-9])(?:-)([0-1][0-9])(?:-)([0-9][0-9][0-9][0-9])(?:\.toml)$",
    ) {
        Ok(r) => r,
        Err(_) => {
            return empty;
        }
    };

    // Go through every file in the $HOME/.cache/crust/ directory. If an error occurs during one of
    // the steps in the loop, go to the next iteration.
    for path in paths {
        let path_os_string = match path {
            Ok(p) => p.file_name(),
            Err(_) => {
                continue;
            }
        };
        let filename = match path_os_string.to_str() {
            Some(f) => f,
            None => {
                continue;
            }
        };

        // Check if the filename matches with the regular expression.
        if re.is_match(&filename) {
            // Capture the date month and year.
            let caps = match re.captures(&filename) {
                Some(c) => c,
                None => {
                    continue;
                }
            };

            let day = match caps[1].parse::<i32>() {
                Ok(d) => d,
                Err(_) => {
                    continue;
                }
            };

            let month = match caps[2].parse::<i32>() {
                Ok(m) => m,
                Err(_) => {
                    continue;
                }
            };

            let year = match caps[3].parse::<i32>() {
                Ok(y) => y,
                Err(_) => {
                    continue;
                }
            };

            // Define the date from the filename.
            let file_date: date::Date = date::Date { day, month, year };

            // Check if the `Date` is valid before pushing it onto the vector.
            if file_date.validate() {
                valid_entries.push(file_date);
            }
        }
    }
    return valid_entries;
}

/// Create and write to file in $HOME/.cache/crust/ to store agenda entries.
pub fn write_entry(program_args: &args::parser::ProgramArguments) {
    // Get the file directory
    let filedir = program_args.date.to_filepath();

    // Open file if it exists, create and open if it does not.
    let mut file = match std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(filedir)
    {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            panic!("Error when attempting to open or create file.");
        }
    };

    // Check if the entry is meant for the entire day, or with time stamps, write the
    // entry accordingly.
    if program_args.flags.full_day {
        match file.write(format!("[[day]]\nevent = '{}'\n", program_args.entry).as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
                panic!("Error when attempting to write to file.");
            }
        };
    } else {
        match file.write(
            format!(
                "[[timestamp]]\nstart = '{}'\nend = '{}'\nevent = '{}'\n",
                program_args.start, program_args.end, program_args.entry
            )
            .as_bytes(),
        ) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
                panic!("Error when attempting to write to file.");
            }
        };
    }
}

/// Open system default `$EDITOR` for date specified by program arguments.
pub fn open_editor(program_args: &args::parser::ProgramArguments) {
    let filepath = program_args.date.to_filepath();
    let editor = match std::env::var("EDITOR") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            panic!("Error obtaining system default $EDITOR.");
        }
    };
    match std::process::Command::new(editor).arg(&filepath).status() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            panic!("Error opening file with default $EDITOR");
        }
    };
}

/// Remove file corresponding to date specified by program arguments.
pub fn remove_file(program_args: &args::parser::ProgramArguments) {
    let filepath = program_args.date.to_filepath();
    if filepath.exists() {
        match std::fs::remove_file(filepath) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
                panic!("Error while attempting to delete file.");
            }
        }
    } else {
        panic!("File to delete does not exist.");
    }
}

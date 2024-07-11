// External Crates
extern crate dirs;
extern crate serde;
extern crate toml;

use serde::{Deserialize, Serialize};
use std::io::Read;

// Structs

#[derive(Deserialize, Serialize, Debug)]
pub struct DayEvent {
    pub event: String,
}

#[derive(Deserialize, Serialize, Debug, Eq, Ord, PartialEq, PartialOrd)] // Add sorting
pub struct TimedEvent {
    pub start: String,
    pub end: String,
    pub event: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(default)] // Use Default impl if field is not encountered in the deserialisation
pub struct Agenda {
    pub day: Vec<DayEvent>,
    pub timestamp: Vec<TimedEvent>,
}

impl Default for Agenda {
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

pub fn get_entry_dir() -> std::path::PathBuf {
    let mut filedir: std::path::PathBuf = dirs::cache_dir().expect("Error obtaining $HOME/.cache");
    filedir.push("crust");
    return filedir;
}

pub fn date_to_filedir(day: i32, month: i32, year: i32) -> std::path::PathBuf {
    let mut filedir: std::path::PathBuf = dirs::cache_dir().expect("Error obtaining $HOME/.cache");

    let day_str: String = {
        if day < 10 {
            format!("0{}", day)
        } else {
            day.to_string()
        }
    };

    let month_str: String = {
        if month < 10 {
            format!("0{}", month)
        } else {
            month.to_string()
        }
    };

    let year_str: String = year.to_string();

    filedir.push(format!("crust/{}-{}-{}.toml", day_str, month_str, year_str));
    return filedir;
}

pub fn parse_agenda_toml(filedir: &mut std::path::PathBuf) -> Agenda {
    let mut file = std::fs::File::open(filedir).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let parsed: toml::Value = toml::from_str(&contents).expect("Error while parsing TOML file");
    let mut toml_struct: Agenda = parsed
        .try_into()
        .expect("Unable to parse TOML into Agenda struct");
    toml_struct.timestamp.sort(); // Sort timed events before printing
    return toml_struct;
}

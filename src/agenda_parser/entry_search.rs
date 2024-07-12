extern crate regex;

use super::parser;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct Date {
    pub day: i32,
    pub month: i32,
    pub year: i32,
}

pub fn get_agenda_entries() -> Vec<Date> {
    // Get the $HOME/.cache/crust/ directory
    let filedir: std::path::PathBuf = parser::get_entry_dir();

    // Get a list of files in that directory
    let paths = std::fs::read_dir(filedir).expect("Unable to get list of agenda entries.");

    // Extract only the filenames and convert then to a Vec of Dates. Simultaneously,
    // check if the filenames are of the valid dd-mm-yyyy.toml structure.
    let mut valid_files: Vec<Date> = Vec::new();
    let re =
        regex::Regex::new(r"^([0-3][0-9])(?:-)([0-1][0-9])(?:-)([0-9][0-9][0-9][0-9])(?:\.toml)$")
            .unwrap();

    for path in paths {
        let path_os_string = path.unwrap().file_name();
        let filename = path_os_string.to_str().unwrap();

        if re.is_match(&filename) {
            let caps = re.captures(&filename).unwrap();
            let date = Date {
                day: caps[1].parse::<i32>().unwrap(),
                month: caps[2].parse::<i32>().unwrap(),
                year: caps[3].parse::<i32>().unwrap(),
            };
            valid_files.push(date);
        }
    }
    return valid_files;
}

pub fn is_between_dates(check_date: &Date, lower_date: &Date, upper_date: &Date) -> bool {
    if check_date > lower_date && check_date < upper_date {
        return true;
    } else {
        return false;
    }
}

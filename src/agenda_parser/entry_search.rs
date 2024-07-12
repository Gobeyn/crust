extern crate regex;

use super::parser;
use crate::calendar_logic::date_conversions;

#[derive(Debug, Clone)]
pub struct Date {
    pub day: i32,
    pub month: i32,
    pub year: i32,
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day && self.month == other.month && self.year == other.year
    }
}

impl Eq for Date {}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            return std::cmp::Ordering::Equal;
        } else {
            if self.year > other.year {
                return std::cmp::Ordering::Greater;
            } else if self.year == other.year {
                if self.month > other.month {
                    return std::cmp::Ordering::Greater;
                } else if self.month == other.month {
                    if self.day > other.day {
                        return std::cmp::Ordering::Greater;
                    } else {
                        return std::cmp::Ordering::Less;
                    }
                } else {
                    return std::cmp::Ordering::Less;
                }
            } else {
                return std::cmp::Ordering::Less;
            }
        }
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
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

pub fn increment_date_by_one(date: &mut Date) {
    let max_day = date_conversions::month_to_days_in_month(date.month, date.year);
    if date.day + 1 > max_day {
        if date.month + 1 > 12 {
            date.day = 1;
            date.month = 1;
            date.year += 1;
        } else {
            date.day = 1;
            date.month += 1;
        }
    } else {
        date.day += 1;
    }
}

pub fn decrement_date_by_one(date: &mut Date) {
    if date.day - 1 <= 0 {
        if date.month - 1 <= 0 {
            date.day = 31;
            date.month = 12;
            date.year -= 1;
        } else {
            let max_day = date_conversions::month_to_days_in_month(date.month - 1, date.year);
            date.day = max_day;
            date.month -= 1;
        }
    } else {
        date.day -= 1;
    }
}

pub fn add_days_to_date(date: &mut Date, days: i32) {
    let mut day_counter = days;
    if days > 0 {
        while day_counter > 0 {
            increment_date_by_one(date);
            day_counter -= 1;
        }
    } else {
        while day_counter < 0 {
            decrement_date_by_one(date);
            day_counter += 1;
        }
    }
}

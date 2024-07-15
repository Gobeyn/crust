// External crates
extern crate chrono;
extern crate dirs;

// Local files
use crate::file;

// Constants
pub const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

pub const DAYS: [&str; 7] = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];

/// Date structure containing the day, month and year as integers.
#[derive(Debug, Copy, Clone)]
pub struct Date {
    pub day: i32,
    pub month: i32,
    pub year: i32,
}

impl Date {
    /// Obtain day of the week from `Date`.
    ///
    /// Method returns an integer from 1 to 7 where `Monday = 1` and `Sunday = 7`.
    /// We use the Zeller's congruence algorithm for Gregorian calendars in this conversion.
    pub fn day_of_week(&self) -> i32 {
        // Make January and February values 13 and 14 instead of 1 and 2, note that they are
        // counted as the 13th and 14th month of the previous year.
        let (month, year) = {
            if self.month == 1 || self.month == 2 {
                (self.month + 12, self.year - 1)
            } else {
                (self.month, self.year)
            }
        };

        // Create auxiliary quantities needed for the computation. We make use of the fact
        // that division with integers keeps only the integer part and ignores the decimal,
        // effectively performing a floor operation without needing to call it.
        let year_of_century = year % 100;
        let zero_based_century = year / 100;

        // Perform the algorithm
        let mut weekday = (self.day
            + (13 * (month + 1) / 5)
            + year_of_century
            + year_of_century / 4
            + zero_based_century / 4
            - 2 * zero_based_century)
            % 7;

        // Shift so Monday = 1 and Sunday = 7, the algorithm returns Saturday = 0 and Friday = 6.
        weekday = ((weekday + 5) % 7) + 1;
        return weekday;
    }

    /// Check if year specified by `Date` is a leap year.
    ///
    /// A leap year is defined as a year that is divisible by 4, but if the year is divisible
    /// by 100 it is not a leap year, unless it is divisible by 400 in which case it is a leap
    /// year.
    pub fn is_leap_year(&self) -> bool {
        if self.year % 4 == 0 {
            if self.year % 100 == 0 {
                if self.year % 400 == 0 {
                    // Divisible by 4, 100 and 400 => Leap year.
                    return true;
                } else {
                    // Divisible by 4 and 100 but not by 400 => Not a leap year.
                    return false;
                }
            } else {
                // Divisible by 4, and not divisible by 100 => Leap year.
                return true;
            }
        } else {
            // Not divisible by 4 => Not a leap year.
            return false;
        }
    }

    /// Obtain maximum number of days in month.
    ///
    /// Provided with a `month` and `year` in the `Date` struct, obtain the maximum amount of
    /// days in that month. We need the year to take the variable number of days in February for
    /// leap years into account.
    pub fn max_day(&self) -> i32 {
        // Check if year is a leap year.
        let leap_year = self.is_leap_year();

        // Check if month is February.
        if self.month == 2 {
            if leap_year {
                return 29;
            } else {
                return 28;
            }
        } else {
            // Otherwise use clever trick that (month - 1) % 7 % 2 returns 0,1,0,1,... for the
            // 30 and 31 day months.
            return 31 - ((self.month - 1) % 7) % 2;
        }
    }

    /// Check if `Date` is a possible date.
    ///
    /// A date is considered valid if
    /// - The year is positive
    /// - The month lies between 1 and 12.
    /// - The day lies between 1 and the maximum amount of days in the given month and year.
    pub fn validate(&self) -> bool {
        // Check that the year is positive.
        if self.year > 0 {
            // Check that 1 <= month <= 12.
            if 1 <= self.month && self.month <= 12 {
                // Check that 1 <= day <= max day in month.
                let m_day = self.max_day();
                if 1 <= self.day && self.day <= m_day {
                    return true;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    /// Increment `Date` by a single day.
    ///
    /// Modify `Date` by going to the next day, wrapping around to the next month or year
    /// when appropriate. Note that this method requires a mutable reference and hence changes the
    /// variable.
    pub fn increment(&mut self) {
        let max_day = self.max_day();
        // Check if we're at the maximum day of the given month.
        if self.day == max_day {
            // If so, check if the next month exceeds December = 12, wrapping to January of the
            // next year if so.
            if self.month == 12 {
                self.day = 1;
                self.month = 1;
                self.year += 1;
            }
            // Otherwise, go to the next month.
            else {
                self.day = 1;
                self.month += 1;
            }
        }
        // Otherwise, increase day by one.
        else {
            self.day += 1;
        }
    }

    /// Decrement `Date` by a single day.
    ///
    /// Modify `Date` by going to the previous day, wrapping around to the previous month or year
    /// when appropriate. Note that this method requires a mutable reference and hence changes the
    /// variable.
    pub fn decrement(&mut self) {
        // Check if we're at the first day of the month.
        if self.day == 1 {
            // Check if the current month is January, if so wrap to December of the previous year.
            if self.month == 1 {
                self.day = 31;
                self.month = 12;
                self.year -= 1;
            }
            // Otherwise, go the last day of the previous month.
            self.month -= 1; // This mutates the month so max_day() should use the mutated month.
            self.day = self.max_day();
        }
        // Otherwise, simply decrease the day by one.
        else {
            self.day -= 1;
        }
    }

    /// Add a number of `days` to the `Date`.
    ///
    /// The amount of days added can be positive or negative and has no bounds. If the `days`
    /// parameter is positive, the `increment` method is called `days` times, conversely,
    /// if `days` is negative the `decrement` method is called `days` times.
    pub fn add_days(&mut self, days: i32) {
        let mut day_counter = days;
        // If we're adding days, increment `days` times.
        if days > 0 {
            while day_counter > 0 {
                self.increment();
                day_counter -= 1;
            }
        }
        // If days is negative, we're subtracting days, so we decrement `days` times.
        else {
            while day_counter < 0 {
                self.decrement();
                day_counter += 1;
            }
        }
        // We get here when the day counter has reached zero, or we wanted to add zero from the
        // start.
    }

    /// Get new instance of `Date` with the first day of the next month.
    ///
    /// Function is useful when we need to know the next month, but don't care about the day.
    pub fn first_of_next_month(&self) -> Self {
        let mut new_date: Date = self.clone();
        new_date.day = 1;

        // If current month is December, wrap around to January of next year.
        if self.month == 12 {
            new_date.month = 1;
            new_date.year += 1;
        }
        // Otherwise, increment the month.
        else {
            new_date.month += 1;
        }

        return new_date;
    }

    /// Obtain $HOME/.cache/crust/dd-mm-yyyy.toml directory from `Date`.
    pub fn to_filepath(&self) -> std::path::PathBuf {
        // Get the $HOME/.cache/ path.
        let mut filedir: std::path::PathBuf =
            dirs::cache_dir().expect("Could not obtain $HOME/.cache");

        // Change to the $HOME/.cache/crust/ path.
        filedir.push("crust");

        // Get the appropriate day format.
        let day_str: String = {
            if self.day < 10 {
                format!("0{}", self.day)
            } else {
                self.day.to_string()
            }
        };

        // Get the appropriate month format.
        let month_str: String = {
            if self.month < 10 {
                format!("0{}", self.month)
            } else {
                self.month.to_string()
            }
        };

        // Get appropriate year format
        let year_str: String = self.year.to_string();

        // Format into the filename
        let filename: String = format!("{}-{}-{}.toml", day_str, month_str, year_str);

        // Add to filepath and return it.
        filedir.push(filename);
        return filedir;
    }

    /// Obtain string containing name of month from `Date`.
    ///
    /// If the `month` value is out of range, an error message is returned instead of the month
    /// name.
    pub fn month_string(&self) -> &str {
        let month_str: &str = {
            let month_shift: usize = (self.month - 1) as usize;
            if month_shift < MONTHS.len() {
                MONTHS[month_shift]
            } else {
                "ERROR: MONTH OUT OF RANGE!"
            }
        };
        return month_str;
    }

    /// Obtain string containing name of day from `Date`
    ///
    /// If the `day` value is out of range, and error message is returned instead of the month
    /// name.
    pub fn day_string(&self) -> &str {
        let weekday = self.day_of_week();
        let day_str: &str = {
            let day_shift: usize = (weekday - 1) as usize;
            if day_shift < DAYS.len() {
                DAYS[day_shift]
            } else {
                "ERROR: DAY OUT OF RANGE!"
            }
        };
        return day_str;
    }

    /// Obtain title for Calendar UI from `Date`.
    pub fn calendar_title(&self) -> String {
        let month_str: &str = self.month_string();
        let title: String = format!(" {} {} ", month_str, self.year);
        return title;
    }

    /// Obtain title for Agenda UI from `Date`.
    pub fn agenda_title(&self) -> String {
        let day_str: &str = self.day_string();
        let month_str: &str = self.month_string();
        let day_end: &str = {
            // 1st, 21st, 31st, etc. 11th is an exception.
            if self.day % 10 == 1 && self.day != 11 {
                "st"
            // 2nd, 22nd, etc. 12th is an exception.
            } else if self.day % 10 == 2 && self.day != 12 {
                "nd"
            // All other numbers end with th.
            } else {
                "th"
            }
        };
        let title: String = format!(
            "  Agenda 󰇙 {}, {} {}{} {} ",
            day_str, month_str, self.day, day_end, self.year
        );
        return title;
    }

    /// Obtain `Agenda` structure from `Date` if the corresponding file in $HOME/.cache/crust/
    /// exists.
    ///
    /// If the dd-mm-yyyy.toml file corresponding to `Date` exists in the $HOME/.cache/crust/,
    /// its contents are parsed into the `Agenda` structure. If it does not exist, we return
    /// `None`.
    pub fn get_agenda(&self) -> Option<file::parser::Agenda> {
        // Get the file directory.
        let mut filedir: std::path::PathBuf = self.to_filepath();

        // Check if file exists, return `None` if not.
        if filedir.exists() {
            return file::parser::parse_agenda_toml(&mut filedir);
        } else {
            return None;
        }
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day && self.month == other.month && self.year == other.year
    }
}

impl Eq for Date {}

impl Ord for Date {
    /// Define what it means for one `Date` to be greater (or lesser) than another `Date`.
    ///
    /// A `Date` is larger than another `Date` if
    /// - The year is larger
    /// - If the year is the same, the month is larger
    /// - If the year and month is the same, the day is larger
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Use the PartialEq definition for equality.
        if self == other {
            return std::cmp::Ordering::Equal;
        } else {
            // Check whose year is larger, making use of Ord on i32.
            if self.year > other.year {
                return std::cmp::Ordering::Greater;
            } else if self.year == other.year {
                // Check whose month is larger.
                if self.month > other.month {
                    return std::cmp::Ordering::Greater;
                } else if self.month == other.month {
                    if self.day > other.day {
                        return std::cmp::Ordering::Greater;
                    }
                    // No need to check equality of days, this case is handled in the beginning.
                    else {
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
    /// Implement `PartialOrd` using `Ord` trait.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for Date {
    /// Current day, month and year as default value for `Date`.
    ///
    /// If it is possible to retrieve and parse the current time into i32 values for the
    /// day, month and year, they are used as the default values. In case an error occurs,
    /// September 16th 2001 is used as the default.
    fn default() -> Self {
        let current_time = std::time::SystemTime::now();
        let datetime: chrono::DateTime<chrono::Utc> = current_time.clone().into();
        let day = match format!("{}", datetime.format("%d")).parse::<i32>() {
            Ok(d) => d,
            Err(_) => 16,
        };
        let month = match format!("{}", datetime.format("%m")).parse::<i32>() {
            Ok(m) => m,
            Err(_) => 9,
        };
        let year = match format!("{}", datetime.format("%Y")).parse::<i32>() {
            Ok(y) => y,
            Err(_) => 2001,
        };
        Date { day, month, year }
    }
}

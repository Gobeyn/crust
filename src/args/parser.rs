// External crates
extern crate getopts;

// Local files
use crate::date::date;

// Structures

/// Store optional flags passed to the program during argument parsing.
#[derive(Clone, Copy, Debug)]
pub struct ProgramFlags {
    pub open_calendar: bool,
    pub full_day: bool,
    pub edit: bool,
    pub remove: bool,
}

impl Default for ProgramFlags {
    /// Default values for `ProgramFlags`, all initialised to `false`.
    fn default() -> Self {
        ProgramFlags {
            open_calendar: false,
            full_day: false,
            edit: false,
            remove: false,
        }
    }
}

/// Stores arguments after argument parsing.
///
/// The `start` and `end` fields are assumed to be of the format `xy:zw`.
#[derive(Clone, Debug)]
pub struct ProgramArguments {
    pub date: date::Date,
    pub flags: ProgramFlags,
    pub start: String,
    pub end: String,
    pub entry: String,
}

impl Default for ProgramArguments {
    /// Default values for `ProgramArguments`
    ///
    /// Values inherent from `Date` and `ProgramFlags` structs. Remaining String fields
    /// initialise to empty strings.
    fn default() -> Self {
        ProgramArguments {
            date: date::Date::default(),
            flags: ProgramFlags::default(),
            start: "".to_string(),
            end: "".to_string(),
            entry: "".to_string(),
        }
    }
}

pub fn parse_arguments() -> ProgramArguments {
    // Get arguments passed to program.
    let args: Vec<String> = std::env::args().collect();

    // Create `ProgramArguments` structure with default values.
    let mut return_args: ProgramArguments = ProgramArguments::default();

    // Define the valid options with `getopts`.
    let mut opts = getopts::Options::new();
    opts.optopt("d", "day", "Enter day for agenda", "DAY [1-31]");
    opts.optopt("m", "month", "Enter month for agenda", "MONTH [1-12]");
    opts.optopt("y", "year", "Enter year for agenda", "YEAR [0-i32 MAX]");
    opts.optopt(
        "a",
        "add",
        "Add entry to current day, or the date specified by the -d, -m and -y 
        arguments.",
        "ENTRY ['STRING']",
    );
    opts.optopt("s", "start", "Starting time of entry", "START [xy:wz]");
    opts.optopt("e", "end", "Ending time of entry", "END [xy:wz]");
    opts.optflag("o", "open", "Open calendar UI");
    opts.optflag(
        "f",
        "full",
        "Entry counts for entire current day, or the date specified by the -d, -m 
        and -y arguments.",
    );
    opts.optflag(
        "",
        "edit",
        "Edit file corresponding to current day, or the date specified by the 
        -d, -m and -y arguments using the system default $EDITOR.",
    );
    opts.optflag(
        "",
        "remove",
        "Remove agenda entry for current day, or the date specified by the 
        -d, -m and -y arguments.",
    );

    // Parse the argument options.
    // Options with arguments.
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e);
            panic!("Invalid arguments.");
        }
    };

    // Update the program arguments for each present option case.
    if matches.opt_present("d") {
        let read_arg = match matches.opt_str("d") {
            Some(v) => v,
            None => {
                panic!("Invalid use of -d argument");
            }
        };
        return_args.date.day = match read_arg.parse::<i32>() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                panic!("Error converting -d argument to integer.");
            }
        };
    }

    if matches.opt_present("m") {
        let read_arg = match matches.opt_str("m") {
            Some(v) => v,
            None => {
                panic!("Invalid use of -m argument");
            }
        };
        return_args.date.month = match read_arg.parse::<i32>() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                panic!("Error converting -m argument to integer.");
            }
        };
    }

    if matches.opt_present("y") {
        let read_arg = match matches.opt_str("y") {
            Some(v) => v,
            None => {
                panic!("Invalid use of -y argument");
            }
        };
        return_args.date.year = match read_arg.parse::<i32>() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                panic!("Error converting -y argument to integer.");
            }
        };
    }

    if matches.opt_present("a") {
        let read_arg = match matches.opt_str("a") {
            Some(v) => v,
            None => {
                panic!("Invalid use of -a argument");
            }
        };
        return_args.entry = read_arg;
    }

    if matches.opt_present("s") {
        let read_arg = match matches.opt_str("s") {
            Some(v) => v,
            None => {
                panic!("Invalid use of -s argument");
            }
        };
        return_args.start = read_arg;
    }

    if matches.opt_present("e") {
        let read_arg = match matches.opt_str("e") {
            Some(v) => v,
            None => {
                panic!("Invalid use of -e argument");
            }
        };
        return_args.end = read_arg;
    }

    // Options that are flags without arguments.
    if matches.opt_present("o") {
        return_args.flags.open_calendar = true;
    }

    if matches.opt_present("f") {
        return_args.flags.full_day = true;
    }

    if matches.opt_present("edit") {
        return_args.flags.edit = true;
    }

    if matches.opt_present("remove") {
        return_args.flags.remove = true;
    }

    // Returns updated arguments
    return return_args;
}

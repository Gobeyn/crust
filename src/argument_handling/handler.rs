// External crates
extern crate chrono;
extern crate getopts;

// Program argument struct

#[derive(Clone, Debug)]
pub struct ProgramArguments {
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub open_calendar: bool,
    pub entry: String,
    pub full_day: bool,
    pub start: String,
    pub end: String,
    pub edit: bool,
    pub remove: bool,
}

impl Default for ProgramArguments {
    fn default() -> Self {
        let current_time = std::time::SystemTime::now();
        let datetime: chrono::DateTime<chrono::Utc> = current_time.clone().into();

        ProgramArguments {
            day: format!("{}", datetime.format("%d")).parse::<i32>().unwrap(),
            month: format!("{}", datetime.format("%m")).parse::<i32>().unwrap(),
            year: format!("{}", datetime.format("%Y")).parse::<i32>().unwrap(),
            open_calendar: false,
            entry: "".to_string(),
            full_day: false,
            start: "".to_string(),
            end: "".to_string(),
            edit: false,
            remove: false,
        }
    }
}

pub fn parse_arguments() -> ProgramArguments {
    // Get arguments passed to the program
    let args: Vec<String> = std::env::args().collect();

    // Create structure with default argument values
    let mut return_args = ProgramArguments::default();

    // Define what the valid options are
    let mut opts = getopts::Options::new();
    opts.optopt("d", "day", "Enter day for calendar.", "DAY");
    opts.optopt("m", "month", "Enter month for calendar", "MONTH");
    opts.optopt("y", "year", "Enter year for calendar", "YEAR");
    opts.optflag("o", "open", "Open the calendar UI");
    opts.optopt(
        "a",
        "add",
        "Add entry to current day, or specified day by --day, --month and --year flags",
        "ENTRY",
    );
    opts.optflag("f", "full", "Added entry counts for entire day");
    opts.optopt(
        "s",
        "start",
        "Starting time of entry, recommended format is xy:zw",
        "TIME",
    );
    opts.optopt(
        "e",
        "end",
        "Ending time of entry, reccommended format is xy:zw",
        "TIME",
    );
    opts.optflag(
        "",
        "edit",
        "Edit file specified by --day, --month and --year flags or the current day if 
        not provided. This uses the 
        default $EDITOR of the system.",
    );
    opts.optflag(
        "",
        "remove",
        "Remove agenda entry specified by --day, --month and --year flags 
        or the current day if not provided.",
    );

    // Parse the options
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e);
            panic!("Unable to parse arguments.")
        }
    };

    // Handle each option case
    if matches.opt_present("d") {
        let read_arg = matches
            .opt_str("d")
            .expect("Unable to extract argument as String");
        return_args.day = read_arg.parse::<i32>().unwrap();
    }
    if matches.opt_present("m") {
        let read_arg = matches
            .opt_str("m")
            .expect("Unable to extract argument as String");
        return_args.month = read_arg.parse::<i32>().unwrap();
    }
    if matches.opt_present("y") {
        let read_arg = matches
            .opt_str("y")
            .expect("Unable to extract argument as String");
        return_args.year = read_arg.parse::<i32>().unwrap();
    }

    if matches.opt_present("o") {
        return_args.open_calendar = true;
    }

    if matches.opt_present("a") {
        let read_args = matches
            .opt_str("a")
            .expect("Unable to extract argument as String");
        return_args.entry = read_args;
    }

    if matches.opt_present("f") {
        return_args.full_day = true;
    }

    if matches.opt_present("s") {
        let read_args = matches
            .opt_str("s")
            .expect("Unable to extract argument as String");
        return_args.start = read_args;
    }

    if matches.opt_present("e") {
        let read_args = matches
            .opt_str("e")
            .expect("Unable to extract arguemtn as String");
        return_args.end = read_args;
    }

    if matches.opt_present("edit") {
        return_args.edit = true;
    }

    if matches.opt_present("remove") {
        return_args.remove = true;
    }

    return return_args;
}

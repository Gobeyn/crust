// External crates

extern crate chrono;
extern crate getopts;

// Program argument struct

#[derive(Debug)]
pub struct ProgramArguments {
    pub day: i32,
    pub month: i32,
    pub year: i32,
}

impl Default for ProgramArguments {
    fn default() -> Self {
        let current_time = std::time::SystemTime::now();
        let datetime: chrono::DateTime<chrono::Utc> = current_time.clone().into();

        ProgramArguments {
            day: format!("{}", datetime.format("%d")).parse::<i32>().unwrap(),
            month: format!("{}", datetime.format("%m")).parse::<i32>().unwrap(),
            year: format!("{}", datetime.format("%Y")).parse::<i32>().unwrap(),
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

    return return_args;
}

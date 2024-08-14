# Crust: Calender CLI in Rust

Crust is a minimal calendar CLI tool. You can enter basic calendar entries in the terminal and view your agenda in a TUI.

## Requirements

To install `crust` you need:

- [Git](https://git-scm.com/downloads)
- [GNU make](https://www.gnu.org/software/make/)
- [Cargo](https://www.rust-lang.org/tools/install)

## Installation

To install `crust`, first clone the repository:
```
$ git clone https://github.com/Gobeyn/crust.git
```
Then, build the application:
```
$ make 
```
Move the binary so it is globally accessible
```
$ make install
```
Finally clean the project
```
$ make clean
```

## Usage

### Program flags

The following contains all program flags for the `crust` binary. 

| Flag | Usage |
| ---- | ----- |
| -o / --open | Open the TUI |
| -d / --day  DAY (1-31) | Day for new entry, or to open TUI on |
| -m / --month MONTH (1-12) | Month for new entry, or to open TUI on |
| -y / --year YEAR (0-?) | Year for new entry, or to open TUI on |
| -a / --add ENTRY (\'TEXT\') | Entry for the specified dd/mm/yyyy, single quotes are suggested |
| -s / --start START (xy:wz) | Starting time of entry, following the suggested format will order the events correctly |
| -e / --end END (xy:wz) | Ending time of entry, following the suggested format will order the events correctly |
| -f / --full | Event counts for the entire day and does not have a start/end time |
| --edit | Edit the events of the specified date using the system default $EDITOR |
| --remove | Remove the entries for the specified date |

### TUI keybinds

The following contains the key binds for movements within the TUI of `crust`.

| Key | Action |
| --- | ------ |
| q | Quit the TUI |
| n | Go to next date in calendar |
| p | Go to previous date in calendar |

### Notes

- By default the day, month and year is the current date. If a day, month or year is not specified by the program flags, then those values 
    assume the default value. For example, suppose you specify a day with the `-d` flag, but leave the month and year unspecified, then 
    the program will default those values to the current month and year.
- The TUI is assumed to be used only for viewing agenda entries, not modifying them. We do not plan on making agenda entries addable or editable from
    the TUI, such operations should be done in the terminal.


## Configuration

Currently not implemented.

## Plans

- Add a configuration file for changing colors and key binds
- Change program flags, the usage of `-a` for adding an entry feels weird, we will most likely replace it with `-m` for 'message' as 
    `git` uses it. Entering dates is also unnecessarily verbose currently.
- Add more TUI movements like next/previous week jumps and next/previous month jumps.
- Add special symbols for holidays.
- Add repeating events.

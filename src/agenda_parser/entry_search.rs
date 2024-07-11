extern crate regex;

use super::parser;

pub fn get_agenda_entries() -> Vec<String> {
    // Get the $HOME/.cache/crust/ directory
    let filedir: std::path::PathBuf = parser::get_entry_dir();

    // Get a list of files in that directory
    let paths = std::fs::read_dir(filedir).expect("Unable to get list of agenda entries.");

    // Extract only the filenames and convert then to a Vec of Strings. Simultaneously,
    // check if the filenames are of the valid dd-mm-yyyy.toml structure.
    let mut valid_files: Vec<String> = Vec::new();
    let re = regex::Regex::new(r"^[0-3][0-9](?:-)[0-1][0-9](?:-)[0-9][0-9][0-9][0-9](?:\.toml)$")
        .unwrap();

    for path in paths {
        let path_os_string = path.unwrap().file_name();
        let path_os_string_copy = path_os_string.clone();

        if re.is_match(path_os_string.to_str().unwrap()) {
            valid_files.push(path_os_string_copy.into_string().unwrap());
        }
    }
    return valid_files;
}

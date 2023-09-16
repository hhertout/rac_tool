use std::process;

pub struct Logger;

impl Logger {
    pub fn error_file_not_found() {
        println!("Error : configuration file not found");
        process::exit(1)
    }
}

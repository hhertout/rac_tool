use std::process;

pub struct Logger;

impl Logger {
    pub fn error_file_not_found() {
        println!("Error : config file path provide is wrong");
        process::exit(1)
    }
}

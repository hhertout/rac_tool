use colored::*;
use std::error::Error;
use std::process;

pub struct Logger;

impl Logger {
    pub fn init_success(filename: &str, destination_path: Option<&str>) {
        let path = match destination_path {
            Some(path) => path.to_owned() + filename,
            None => filename.to_owned(),
        };
        println!("{}", "Initialisation file successfully created".green());
        println!(
            "{}{}",
            "Your config file is now located at : ./".green(),
            path.green()
        )
    }
    pub fn init_failed<'a>(err: &dyn Error) {
        println!("{}", "Init failed".red());
        println!("{}", err);
        process::exit(1);
    }
    pub fn wrong_args() {
        println!("{}", "Error : You provide a wrong argument".red());
        process::exit(1);
    }
    pub fn error_file_not_found() {
        println!("{}", "Error : configuration file not found".red());
        process::exit(1)
    }
    pub fn yml_parse_failed() {
        println!("{}", "Error : Convertion to yaml failed.".red());
    }
    pub fn create_file_failure() {
        println!("{}", "Error : Failed to create file.".red())
    }
    pub fn copy_success(from: &str, to: &str) {
        println!("Successfully cpoy {} to {}", from, to);
    }
}

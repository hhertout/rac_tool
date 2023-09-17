use std::fs;

use logger::Logger;
use schema::Schema;
use std::io::Error;

use crate::YamlParser;

pub struct Replacer {
    config_file_path: String,
}

impl YamlParser for Replacer {
    fn parse_yml(&self) -> Schema {
        let content = fs::read_to_string(&self.config_file_path);
        if content.is_err() {
            Logger::error_file_not_found();
        }
        let content_unwrap = content.unwrap();
        match serde_yaml::from_str(&content_unwrap) {
            Ok(parsed_content) => parsed_content,
            Err(_) => {
                Logger::invalid_config_file();
                panic!("Process exited")
            }
        }
    }
}

impl Replacer {
    pub fn new(config_file_path: String) -> Replacer {
        Replacer { config_file_path }
    }

    pub fn run_replace(&self, sentence: String, file_path: &str) -> Result<(), Error> {
        let splited: Vec<&str> = sentence.split(":").collect();
        let metadata = fs::metadata(file_path).expect("Unreadable directory or file");
        if !metadata.is_file() {
            return Ok(())
        }
        let content = fs::read_to_string(file_path);
        match content {
            Ok(content) => {
                if !content.contains(splited[0]) {
                    return Ok(());
                }
                let new_content = content.replace(splited[0], splited[1]);
                match fs::write(file_path, &new_content) {
                    Ok(_) => {
                        Logger::string_successfully_replaced(file_path);
                        Ok(())
                    },
                    Err(err) => {
                        Logger::string_replaced_err(file_path);
                        return Err(err)
                    },
                }
            }
            Err(err) => return Err(err),
        }
    }
}

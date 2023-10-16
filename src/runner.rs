use copier::file_copier::FileCopier;
use copier::replacer::Replacer;
use copier::YamlParser;
use std::io::Result;
use std::{fs, path::Path};

use logger::Logger;
use schema::Schema;

pub struct Runner {
    config_file_name: String,
    copier: FileCopier,
    replacer: Replacer,
    schema: Option<Schema>,
}

impl Runner {
    pub fn new(config_file: String) -> Runner {
        Runner {
            config_file_name: config_file.clone(),
            copier: FileCopier::new(config_file.clone()),
            replacer: Replacer::new(config_file.clone()),
            schema: None,
        }
    }

    pub fn run(&mut self) {
        self.schema = Some(self.copier.parse_yml());
        let schema = self.schema.as_ref().unwrap();
        if schema.copy.is_some() {
            self.copy();
        }
        if schema.replace.is_some() {
            self.replace();
        }
    }

    pub fn copy(&self) {
        let schema = self.schema.clone().expect("Error: Empty schema");
        let dir = Path::new(&schema.on);
        let _ = self.visit_dir_and_copy(dir, &schema);
    }

    pub fn replace(&self) {
        let schema = self.schema.clone().expect("Error: Empty schema");
        let dir = Path::new(&schema.on);
        let _ = self.visit_dir_and_replace(dir, &schema);
    }

    pub fn visit_dir_and_replace(&self, path: &Path, schema: &Schema) -> Result<()> {
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        let entry_path = path.as_os_str().to_str().unwrap();
                        if self.is_dir_is_ignored(entry_path) {
                            continue;
                        };
                        let replace = schema.replace.clone().unwrap();
                        if replace.global.is_some() {
                            for sentence in replace.global.unwrap() {
                                let _ = self.replacer.run_replace(sentence, entry_path);
                            }
                        }
                        if replace.target.is_some() {
                            for file in replace.target.unwrap() {
                                if entry_path.ends_with(&file.file_name) {
                                    for sentence in file.content {
                                        let _ = self.replacer.run_replace(sentence, entry_path);
                                    }
                                }
                            }
                        }
                        let _ = self.visit_dir_and_replace(&path, schema);
                    }
                    Err(_) => {
                        Logger::dir_unavailable();
                    }
                }
            }
        }
        Ok(())
    }

    pub fn visit_dir_and_copy(&self, path: &Path, schema: &Schema) -> Result<()> {
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        let entry_path = path.as_os_str().to_str().unwrap();
                        if self.is_dir_is_ignored(entry_path) {
                            continue;
                        };
                        let copy = schema.copy.clone().unwrap();
                        for file in copy.files {
                            let file_split: Vec<&str> = file.split(":").collect();
                            if entry_path.contains(file_split[0]) {
                                let result_file_name =
                                    entry_path.replace(file_split[0], file_split[1]);
                                let _ = &self.copier.run_copy(entry_path, &result_file_name);
                            }
                        }
                        let _ = self.visit_dir_and_copy(&path, schema);
                    }
                    Err(_) => {
                        Logger::dir_unavailable();
                    }
                }
            }
        }
        Ok(())
    }

    fn is_dir_is_ignored(&self, path: &str) -> bool {
        if path.contains(&self.config_file_name) {
            return true;
        }
        match self.schema.clone() {
            Some(schema) => match schema.ignored_dir {
                Some(ignored_dir) => {
                    for dir in ignored_dir {
                        if path.contains(&dir) {
                            return true;
                        }
                    }
                    return false;
                }
                None => false,
            },
            None => false,
        }
    }
}

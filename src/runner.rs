use std::io::Result;
use std::{fs::read_dir, path::Path};

use copier::Copier;
use schema::Schema;

pub struct Runner {
    copier: Copier,
    schema: Option<Schema>,
}

impl Runner {
    pub fn new(filename: String) -> Runner {
        Runner {
            copier: Copier::new(filename),
            schema: None,
        }
    }

    pub fn run(&mut self) {
        self.schema = Some(self.copier.parse_yml());
        let schema = self.schema.clone().expect("empty schema");
        let dir = Path::new(&schema.on);
        let _ = self.visit_dir(dir, &schema);
    }

    pub fn visit_dir(&self, path: &Path, schema: &Schema) -> Result<()> {
        if path.is_dir() {
            for entry in read_dir(path)? {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        let files = &schema.files;
                        let entry_path = path.as_os_str().to_str().unwrap();
                        if self.is_dir_is_ignored(entry_path) {
                            continue;
                        };
                        for file in files {
                            let file_split: Vec<&str> = file.split(":").collect();
                            if entry_path.contains(file_split[0]) {
                                let result_file_name =
                                    entry_path.replace(file_split[0], file_split[1]);
                                let _ = &self.copier.run_copy(entry_path, &result_file_name);
                            }
                        }
                        let _ = self.visit_dir(&entry.path(), schema);
                    }
                    Err(err) => {
                        println!("{}", err)
                    }
                }
            }
        }
        Ok(())
    }

    fn is_dir_is_ignored(&self, path: &str) -> bool {
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

use std::io::Result;
use std::{fs::read_dir, path::Path};

use copier::Copier;
use schema::Schema;

pub struct Runner {
    copier: Copier,
}

impl Runner {
    pub fn new(filename: String) -> Runner {
        Runner {
            copier: Copier::new(filename),
        }
    }

    pub fn run(&self) {
        let schema: Schema = self.copier.parse_yml();
        let dir = Path::new(&schema.on);
        self.visit_dir(dir, &schema);
    }

    pub fn visit_dir(&self, path: &Path, schema: &Schema) -> Result<()> {
        let ignored_dir: Vec<&str> = vec![
            "./target",
            "./.git",
            "./github",
            "./node_modules",
            "./vendor",
            "./public",
        ];

        if path.is_dir() {
            for entry in read_dir(path)? {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        println!("{:?}", &entry.path());
                        if !ignored_dir.contains(&entry.path().as_os_str().to_str().unwrap()) {
                            let files = &schema.files;
                            let entry_file_name =
                                path.file_name().unwrap().to_str().unwrap().clone();
                            for file in files {
                                let file_split: Vec<&str> = file.split(":").collect();
                                if file_split[0] == entry_file_name {
                                    let _ = &self.copier.run_copy(file_split[0], file_split[1]);
                                }
                            }
                            //let _ = self.visit_dir(&entry.path(), schema);
                        }
                    }
                    Err(err) => {
                        println!("{}", err)
                    }
                }
            }
        }
        Ok(())
    }
}

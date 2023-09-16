use schema::Schema;
use std::fs::File;
use std::io::Write;

pub struct Initializer {
    pub filename: String,
    pub schema: Schema,
}

impl Initializer {
    pub fn new(filename: String) -> Initializer {
        Initializer {
            filename,
            schema: Schema::new(),
        }
    }
    fn convert_to_yml(&self) -> String {
        let yaml = serde_yaml::to_string(&self.schema);
        match yaml {
            Ok(result) => result,
            Err(err) => panic!("Error : Convertion to yaml failed. {}", err),
        }
    }

    pub fn create_yml(&self, destiniation_path: Option<&str>) -> Result<(), std::io::Error> {
        let yaml = self.convert_to_yml();
        let dest = match destiniation_path {
            Some(dest) => dest.to_owned() + self.filename.as_str(),
            None => String::from("./") + self.filename.as_str(),
        };
        let mut file = match File::create(dest) {
            Ok(file) => file,
            Err(err) => panic!("Error : Failed to create file. {}", err),
        };
        file.write_all(yaml.as_bytes())
    }
}

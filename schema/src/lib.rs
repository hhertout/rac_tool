use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Schema {
    pub on: String,
    pub files: Vec<String>,
    pub ingored_dir: Option<Vec<String>>,
}

impl Schema {
    pub fn new() -> Schema {
        Schema {
            on: String::from("."),
            files: vec![String::from("hello.txt.example:hello.txt")],
            ingored_dir: Some(vec![
                String::from("/.git/"),
                String::from("/node_modules/"),
                String::from("/vendor/"),
                String::from("/target/"),
            ]),
        }
    }
}

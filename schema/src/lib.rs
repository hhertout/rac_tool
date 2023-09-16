use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Schema {
    pub on: String,
    pub files: Vec<String>
}

impl Schema {
    pub fn new() -> Schema {
        Schema {
            on: String::from("."),
            files: vec![String::from("hello.txt.example:hello.txt")]
        }
    }
}

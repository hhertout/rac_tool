use std::vec;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Schema {
    pub on: String,
    pub copy: Option<Copy>,
    pub replace: Option<Replace>,
    pub ignored_dir: Option<Vec<String>>,
}

impl Schema {
    pub fn new() -> Schema {
        Schema {
            on: String::from("."),
            copy: Some(Copy::new()),
            replace: Some(Replace::new()),
            ignored_dir: Some(vec![
                String::from("/.git/"),
                String::from("/node_modules/"),
                String::from("/vendor/"),
                String::from("/target/"),
            ]),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Copy {
    pub files: Vec<String>,
}

impl Copy {
    fn new() -> Copy {
        Copy {
            files: vec![String::from("hello.txt.example:hello.txt")],
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Replace {
    pub global: Vec<String>,
    pub target: Vec<Target>,
}

impl Replace {
    fn new() -> Replace {
        Replace {
            global: vec![String::from("hello world:hello mom")],
            target: vec![
                Target {
                    file_name: "hello.txt".to_owned(),
                    content: "hello world:hello mom".to_owned(),
                },
                Target {
                    file_name: "dir/example/hello.txt".to_owned(),
                    content: "hello world:hello mom".to_owned(),
                },
            ],
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Target {
    file_name: String,
    content: String,
}

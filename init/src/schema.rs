use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Schema {
    foo: String,
}

impl Schema {
    pub fn new() -> Schema {
        Schema {
            foo: String::from("bar"),
        }
    }
}

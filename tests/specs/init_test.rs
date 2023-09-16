use std::fs;

use init::{Initializer, schema::Schema};

#[test]
pub fn create_yaml() {
    let filename = String::from("config_test.yml");
    let initializer = Initializer::new(filename.clone());
    let result = initializer.create_yml(Some("./tests/specs/yaml/"));
    assert!(result.is_ok());

    let path = "./tests/specs/yaml/".to_owned() + &filename;
    let content = fs::read_to_string(path);
    assert!(content.is_ok());
    let c = content.unwrap();
    let obj: Schema = serde_yaml::from_str(&c).unwrap();
    assert_eq!(obj, initializer.schema)
}

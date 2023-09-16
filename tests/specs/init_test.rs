use std::fs;

use init::Initializer;

#[test]
pub fn create_yaml() {
    let filename = String::from("config.yml");
    let initializer = Initializer::new(filename.clone());
    let result = initializer.create_yml(Some("./tests/specs/yaml/"));
    assert!(result.is_ok());

    let path = "./tests/specs/yaml/".to_owned() + &filename;
    let content = fs::read_to_string(path);
    assert!(content.is_ok());
}

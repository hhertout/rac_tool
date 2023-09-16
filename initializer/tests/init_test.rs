use initializer::Initializer;
use schema::Schema;
use std::fs;

#[test]
pub fn create_yaml() {
    let filename = String::from("config_test.yml");
    let initializer = Initializer::new(filename.clone());
    let result = initializer.create_yml(Some("./tests/yaml/"));
    assert!(result.is_ok());

    let path = "./tests/yaml/".to_owned() + &filename;
    let content = fs::read_to_string(path);
    assert!(content.is_ok());
    let c = content.unwrap();
    let obj: Schema = serde_yaml::from_str(&c).unwrap();
    assert_eq!(obj, initializer.schema)
}

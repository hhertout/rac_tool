use copier::file_copier::FileCopier;
use copier::YamlParser;
use initializer::Initializer;
use schema::Schema;
use std::fs;

fn create_config() -> Initializer {
    let filename = String::from("config_test.yml");
    let initializer = Initializer::new(filename.clone());
    let result = initializer.create_yml(Some("./tests/yaml/"));
    assert!(result.is_ok());

    return initializer;
}

#[test]
pub fn init() {
    create_config();
    let config_file_path = String::from("./tests/yaml/config_test.yml");
    let copier = FileCopier::new(config_file_path.clone());
    assert_eq!(copier.config_file_path, config_file_path.clone());
}

#[test]
pub fn copier_retrieve_schema() {
    let initializer = create_config();
    let config_file_path = String::from("./tests/yaml/config_test.yml");
    let copier = FileCopier::new(config_file_path.clone());
    assert_eq!(copier.config_file_path, config_file_path.clone());

    let schema: Schema = copier.parse_yml();

    assert_eq!(initializer.schema, schema)
}

#[test]
pub fn copier_run_test() {
    let initializer = create_config();
    let config_file_path = String::from("./tests/yaml/config_test.yml");
    let copier = FileCopier::new(config_file_path.clone());
    assert_eq!(copier.config_file_path, config_file_path.clone());

    let schema: Schema = copier.parse_yml();

    assert_eq!(initializer.schema, schema);

    copier.run_copy(
        "./tests/playground/text.txt.example",
        "./tests/playground/text.txt",
    );

    let file_exist = fs::metadata("./tests/playground/text.txt");
    assert!(file_exist.is_ok());

    let content_from = fs::read_to_string("./tests/playground/text.txt.example");
    let content_to = fs::read_to_string("./tests/playground/text.txt");
    assert_eq!(content_from.unwrap(), content_to.unwrap());
}

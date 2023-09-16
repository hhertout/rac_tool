use copier::Copier;
use init::Initializer;
use schema::Schema;

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
    let copier = Copier::new(config_file_path.clone());
    assert_eq!(copier.config_file_path, config_file_path.clone());
}

#[test]
pub fn copier_retrieve_schema() {
    let initializer = create_config();
    let config_file_path = String::from("./tests/yaml/config_test.yml");
    let copier = Copier::new(config_file_path.clone());
    assert_eq!(copier.config_file_path, config_file_path.clone());

    let schema: Schema = copier.parse_yml();

    assert_eq!(initializer.schema, schema)
}

#[test]
pub fn copier_run_test() {}

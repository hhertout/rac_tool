use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct YamlTest {
    foo: String,
    hello: Option<String>,
}

#[test]
fn test_write_yaml() {
    let test = YamlTest {
        foo: String::from("bar"),
        hello: None
    };
    let yaml = serde_yaml::to_string(&test).unwrap();

    assert_eq!(yaml, "foo: bar\nhello: null\n");
}

#[test]
fn test_read_yaml() {
    let base_url = String::from("tests/specs/");
    let content = std::fs::read_to_string(base_url + "test.yml").expect("Error : File not found");
    assert_eq!(content, "foo: bar");

    let deserialized = serde_yaml::from_str(&content).unwrap();
    let yaml = YamlTest {
        foo: String::from("bar"),
        hello: None,
    };
    assert_eq!(yaml, deserialized);
}

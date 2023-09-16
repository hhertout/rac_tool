use serde::{Deserialize, Serialize};
use std::{fs::read_dir, path::Path};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct YamlTest {
    foo: String,
    hello: Option<String>,
}

#[test]
fn test_write_yaml() {
    let test = YamlTest {
        foo: String::from("bar"),
        hello: None,
    };
    let yaml = serde_yaml::to_string(&test).unwrap();

    assert_eq!(yaml, "foo: bar\nhello: null\n");
}

#[test]
fn test_read_yaml() {
    let base_url = String::from("tests/specs/yaml/");
    let content = std::fs::read_to_string(base_url + "test.yml").expect("Error : File not found");
    assert_eq!(content, "foo: bar");

    let deserialized = serde_yaml::from_str(&content).unwrap();
    let yaml = YamlTest {
        foo: String::from("bar"),
        hello: None,
    };
    assert_eq!(yaml, deserialized);
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Res {
    on: String,
    files: Vec<String>,
}

#[test]
fn test_yml() {
    let content = "on: .\nfiles:\n  - text.txt.example:text.txt\n".to_owned();
    let deserialized: Res = serde_yaml::from_str(&content).unwrap();
    let split: Vec<&str> = deserialized.files[0].split(":").collect();

    assert_eq!(split[0], "text.txt.example");
    assert_eq!(split[1], "text.txt");
}

#[test]
fn test_dir() -> std::io::Result<()> {
    let ignored_dir: Vec<&str> = vec![
        "./target",
        "./.git",
        "./github",
        "./node_modules",
        "./vendor",
        "./public",
    ];
    let path = Path::new(".");
    if path.is_dir() {
        for entry in read_dir(path)? {
            match entry {
                Ok(entry) => {
                    if !ignored_dir.contains(&entry.path().as_os_str().to_str().unwrap()) {
                        println!("{:?}", entry.path().file_name().unwrap());
                    }
                }
                Err(err) => {
                    println!("{}", err)
                }
            }
        }
    }
    Ok(())
}

#[test]
fn visit_dir_test() {
    let path = Path::new("./tests");
}

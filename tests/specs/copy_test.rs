use std::fs;

use fs_rust::runner::Runner;

#[test]
fn visit_dir_test_with_ignored_path() {
    let mut runner = Runner::new("./tests/playground/config_test.yml".to_owned());
    runner.run();

    let first_file = fs::metadata("./tests/playground/hello.txt");
    println!("{:?}", first_file);
    let second_file = fs::metadata("./tests/playground/random/hello_world.txt");
    println!("{:?}", second_file);
    let error_file = fs::metadata("./tests/playground/ignored/hello.txt");
    println!("{:?}", error_file);
    assert!(first_file.is_ok());
    assert!(second_file.is_ok());
    assert!(error_file.is_err());

    let _ = fs::remove_file("./tests/playground/random/hello_world.txt");
    let _ = fs::remove_file("./tests/playground/hello.txt");
}

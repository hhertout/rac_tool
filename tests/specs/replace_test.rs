use std::fs;

use fs_rust::runner::Runner;

#[test]
fn replace_global_sentence() {
    let _ = fs::remove_file("./tests/playground/random/hello_world.txt");
    let _ = fs::remove_file("./tests/playground/hello.txt");

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

    let content = fs::read_to_string("./tests/playground/hello.txt").unwrap();
    let content_bis = fs::read_to_string("./tests/playground/random/hello_world.txt").unwrap();
    assert_eq!(content, "this is an example - hello mom");
    assert_eq!(content_bis, "this is an other example file");

    let _ = fs::remove_file("./tests/playground/random/hello_world.txt");
    let _ = fs::remove_file("./tests/playground/hello.txt");
}

#[test]
fn replace_target_sentence() {
    let _ = fs::write(
        "./tests/playground/random/targeted_file.txt",
        "sentence to replace",
    );
    let mut runner = Runner::new("./tests/playground/config_test.yml".to_owned());
    runner.run();

    let content = fs::read_to_string("./tests/playground/random/targeted_file.txt").unwrap();
    assert_eq!(content, "sentence replaced");

    let _ = fs::remove_file("./tests/playground/random/hello_world.txt");
    let _ = fs::remove_file("./tests/playground/hello.txt");
}

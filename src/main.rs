use std::env;
use std::mem;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let main_arg = args[1].clone();
    mem::drop(args);

    match main_arg.as_str() {
        "init" => println!("this is init"),
        _ => {
            println!("Error : You provide a wrong arg");
            process::exit(1);
        }
    }
}

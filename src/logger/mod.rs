pub struct Logger;

impl Logger {
    pub fn init_start() {
        println!("==== Start Initialisation ====");
    }
    pub fn init_succes(destination_path: Option<&str>) {
        let path = match destination_path {
            Some(path) => path.to_owned() + "init.yml",
            None => "./init.yaml".to_owned()
        };
        println!("Initialisation file successfully created");
        println!("Your config file is now located at : {}", path)
    }
}
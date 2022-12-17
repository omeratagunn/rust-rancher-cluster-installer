use crate::types::Config;
use colored::Colorize;
use std::fs::File;
use std::path::Path;
use std::process::exit;

pub fn parse_yaml_config(path_to_yaml: &String) -> Config {
    let file_path = Path::new(&path_to_yaml);

    if !file_path.is_file() {
        println!("{}", "\nThis is not a file".red().bold());
        exit(1);
    }

    let file = File::open(&path_to_yaml).expect("file could not be opened");

    let deserialized_map: Config = serde_yaml::from_reader(file).expect("reader");
    return deserialized_map;
}

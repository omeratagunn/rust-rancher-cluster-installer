extern crate yaml_rust;

use yaml_rust::{YamlLoader, Yaml};
use std::fs::File;
use std::io::Read;
use std::string::String;
use std::path::Path;
use std::process::exit;

use crate::app::config::types::{ServerConfigurationArgs, YamlInput};

#[path = "../shared-types/types.rs"] mod types;

pub(crate) fn parse_yaml_config(path_to_yaml: String) -> Vec<Yaml> {
    let file_path = Path::new(&path_to_yaml);

    if  !file_path.is_file() {
        println!("this is not a file");
        exit(1);
    }
    let mut file = File::open(&path_to_yaml).expect("file could not be opened");

    let mut contents = String::new();
    let _content = file.read_to_string(&mut contents);
    let docs: Vec<Yaml> = YamlLoader::load_from_str(&mut contents).unwrap();
    
    return docs;

}



use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::iter::Enumerate;
use std::string::String;
use std::path::Path;
use std::process::exit;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config{
    masters: Vec<ServerConf>,
    nodes: Vec<ServerConf>,
}
#[derive(Debug, Serialize, Deserialize)]
struct ServerConf {
    name: String,
    ip: String,
    username: String,
    password: String
}

#[path = "../shared-types/types.rs"] mod types;

pub(crate) fn parse_yaml_config(path_to_yaml: String) -> Config {
    let file_path = Path::new(&path_to_yaml);

    if  !file_path.is_file() {
        println!("this is not a file");
        exit(1);
    }

    let mut file = File::open(&path_to_yaml).expect("file could not be opened");

    let deserialized_map:Config = serde_yaml::from_reader(file).expect("reader");
    return deserialized_map;

}

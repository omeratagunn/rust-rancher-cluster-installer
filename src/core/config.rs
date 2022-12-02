#![allow(unused)]
extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};
use std::fs::File;
use std::io::Read;

use std::path::Path;
use std::process::exit;

pub(crate) fn parse_yaml_config(path_to_yaml: String) {
    dbg!(&path_to_yaml);
    let file_path = Path::new(&path_to_yaml);

    println!("{path_to_yaml}");
    if  !file_path.is_file() {
        println!("this is not a file");
        exit(1);
    }
    let mut file = File::open(&path_to_yaml).expect("file could not be opened");

    let mut contents = String::new();
    let content = file.read_to_string(&mut contents);
    println!("{:?}", content);
    let docs = YamlLoader::load_from_str(&mut contents).unwrap();
    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];
    println!("{:?}", doc);
    dbg!(&doc["masters"]["server1"][0]);

    // assert_eq!(doc["masters"])
    //
    // // Index access for map & array
    // assert_eq!(doc["masters"][0].as_str().unwrap(), "server1");
    //
    // // Chained key/array access is checked and won't panic,
    // // return BadValue if they are not exist.
    // assert!(doc["INVALID_KEY"][100].is_badvalue());
    //
    // // Dump the YAML object
    // let mut out_str = String::new();
    // {
    //     let mut emitter = YamlEmitter::new(&mut out_str);
    //     emitter.dump(doc).unwrap(); // dump the YAML object to a String
    //
    // }
    // println!("{}", out_str);
}

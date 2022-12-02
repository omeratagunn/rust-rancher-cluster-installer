use std::io;
use std::io::Write;
use crate::app::config::parse_yaml_config;
use colored::*;

#[path = "../utils/measureable.rs"] mod measure;
#[path = "config.rs"] mod config;
#[path = "../utils/spinner.rs"] mod spinner;
#[path = "../utils/sanitize.rs"] mod sanitize;



pub(crate) fn app() {
        let mut user_input = String::new();
        print!("{}!", "- Absolute path to yaml file: ".blue().bold());
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        println!("{}{}", "- Processing ".green().bold(), user_input);
        let spinner_handle = spinner::spinner(" Initiated...".parse().unwrap());
        sanitize::strip_trailing_nl(&mut user_input);

        parse_yaml_config(user_input)

}

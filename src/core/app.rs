use std::io;
use std::io::Write;
use crate::app::config::parse_yaml_config;
use colored::*;
use crate::app::types::{ServerConfigurationArgs, SshCredArgs};

#[path = "../utils/measureable.rs"] mod measure;
#[path = "config.rs"] mod config;
#[path = "../utils/spinner.rs"] mod spinner;
#[path = "../utils/sanitize.rs"] mod sanitize;
#[path = "../core/ssh.rs"] mod ssh;
#[path = "../shared-types/types.rs"] mod types;


pub(crate) fn app() {
        let mut user_input = String::new();
        print!("{}!", "- Absolute path to yaml file: ".blue().bold());
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        println!("{}{}", "- Processing ".green().bold(), user_input);
        // start work indicator //
        let spinner_handle = spinner::spinner(" Initiated...".parse().expect("spinner working"));

        sanitize::strip_trailing_nl(&mut user_input);

        let parsed_yaml =  parse_yaml_config(user_input);

        spinner_handle.done();

        let spinner_handle = spinner::spinner(" Initiated...".parse().expect("spinner working"));
        let server_vec:SshCredArgs = SshCredArgs{
                cred: ServerConfigurationArgs {
                        ip: "".to_string(),
                        name: "digitalocean".to_string(),
                        username: "".to_string(),
                        password: "".to_string()
                },
        };
        let ssh = ssh::connect_server_via_ssh(&server_vec);

        spinner_handle.done();
}


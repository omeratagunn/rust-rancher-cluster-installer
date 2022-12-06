use std::io;
use std::io::Write;
use crate::app::config::parse_yaml_config;
use colored::*;
use crate::app::curl::install;
use crate::app::installation::{Installation, LinuxInstructions};
use crate::app::types::{ServerConfigurationArgs, SshCredArgs};

#[path = "../utils/measureable.rs"]
mod measure;
#[path = "config.rs"]
mod config;
#[path = "../utils/spinner.rs"]
mod spinner;
#[path = "../utils/sanitize.rs"]
mod sanitize;
#[path = "../core/ssh.rs"]
mod ssh;
#[path = "../shared-types/types.rs"]
mod types;
#[path = "curl.rs"]
mod curl;
#[path = "../config/installation.rs"]
mod installation;

pub(crate) fn app(mut path: String) {
    let spinner_handle = spinner::spinner("Parsing yaml file...".parse().expect("spinner working"));

    let parsed_yaml = parse_yaml_config(path);
    spinner_handle.done();
    println!("{:?}", parsed_yaml);

        let spinner_handle = spinner::spinner("Connecting to server...".parse().expect("spinner working"));
        let mut server_vec: SshCredArgs = SshCredArgs {
            cred: ServerConfigurationArgs {
                ip: "".to_string(),
                name: "digitalocean".to_string(),
                username: "root".to_string(),
                password: "".to_string(),
            },
        };

        let ssh_session = ssh::connect_server_via_ssh(&server_vec);
        spinner_handle.done();
        for instructions in installation::get_installation().linux_amd64 {
            install(instructions, &ssh_session);
        }
}


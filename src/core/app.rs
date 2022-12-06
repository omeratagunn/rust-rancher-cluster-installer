use std::io;
use std::io::Write;
use crate::app::config::parse_yaml_config;
use colored::*;
use crate::app::curl::install;
use crate::app::installation::{Installation, LinuxInstructions};
use crate::app::types::{ServerConfigurationArgs, SshCredArgs};

#[path = "../utils/measureable.rs"] mod measure;
#[path = "config.rs"] mod config;
#[path = "../utils/spinner.rs"] mod spinner;
#[path = "../utils/sanitize.rs"] mod sanitize;
#[path = "../core/ssh.rs"] mod ssh;
#[path = "../shared-types/types.rs"] mod types;
#[path = "curl.rs"] mod curl;
#[path = "../config/installation.rs"] mod installation;

pub(crate) fn app(mut path: String) {


        let installation: Installation = Installation {
                linux_amd64: Vec::from([
                        LinuxInstructions {
                                name: "curl".to_string(),
                                command: "curl --version".to_string(),
                                fallback_command: "apt install curl -y".to_string(),
                        },
                        LinuxInstructions {
                                name: "jq".to_string(),
                                command: "jq --version".to_string(),
                                fallback_command: "apt-get update && apt-get install jq -y".to_string(),
                        }])
        };


        let spinner_handle = spinner::spinner(" Parsing yaml file...".parse().expect("spinner working"));

        //let parsed_yaml =  parse_yaml_config(path);
        spinner_handle.done();

        let spinner_handle = spinner::spinner(" Connecting to server...".parse().expect("spinner working"));
        let server_vec:SshCredArgs = SshCredArgs{
                cred: ServerConfigurationArgs {
                        ip: "134.122.84.135:22".to_string(),
                        name: "digitalocean".to_string(),
                        username: "root".to_string(),
                        password: "oL2SUTYV3A0ASo7x".to_string()
                },
        };
        let ssh_session = ssh::connect_server_via_ssh(&server_vec);
        spinner_handle.done();

        for instructions in installation.linux_amd64{
                install(instructions, &ssh_session);
        }






}


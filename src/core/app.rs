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
#[path = "curl.rs"] mod curl;
#[path = "longhorn.rs"] mod longhorn;

pub(crate) fn app(mut path: String) {

        let spinner_handle = spinner::spinner(" Parsing yaml file...".parse().expect("spinner working"));

        //let parsed_yaml =  parse_yaml_config(path);
        spinner_handle.done();

        let spinner_handle = spinner::spinner(" Connecting to server...".parse().expect("spinner working"));

        let ssh_session = ssh::connect_server_via_ssh(&server_vec);
        spinner_handle.done();

        curl::check_curl(&ssh_session);

        longhorn::install_longhorn_reqs(&ssh_session);



}


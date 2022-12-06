
use crate::app::yaml::parse_yaml_config;

#[path = "../utils/measureable.rs"]
mod measure;
#[path = "yaml.rs"]
mod yaml;
#[path = "../utils/spinner.rs"]
mod spinner;
#[path = "../utils/sanitize.rs"]
mod sanitize;
#[path = "../core/ssh.rs"]
mod ssh;
#[path = "installation.rs"]
mod install;
#[path = "../config/config.rs"]
mod installation;
#[path = "yaml.rs"]
mod config;

pub(crate) fn app( path: String) {
    let spinner_handle = spinner::spinner("Parsing yaml file...".parse().expect("spinner working"));

    let parsed_yaml = parse_yaml_config(path);
    spinner_handle.done();

    for masters in parsed_yaml.masters {
        let spinner_handle = spinner::spinner("Connecting to server... ".parse().expect("spinner working"));
        let ssh_session = ssh::connect_server_via_ssh(&masters);
        spinner_handle.done();
        for instructions in installation::get_installation().linux_amd64 {
            install::install(instructions, &ssh_session);
        }
    }
}


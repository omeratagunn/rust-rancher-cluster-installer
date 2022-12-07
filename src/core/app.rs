use std::fs;
use std::path::Path;
use colored::Colorize;
use crate::app::yaml::{Config, parse_yaml_config};

#[path = "../utils/measurable.rs"]
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


pub(crate) fn app( path: &String, k3s_version: &String) {
    let spinner_handle = spinner::spinner("Parsing yaml file...".parse().expect("spinner working"));

    let parsed_yaml = parse_yaml_config(path);
    spinner_handle.done();

    build_masters(&parsed_yaml, k3s_version);
    build_nodes(&parsed_yaml, k3s_version);

}

fn build_masters(masters: &Config, k3s_version: &String){
    for (master_node_index,masters) in masters.masters.iter().enumerate() {
        let spinner_handle = spinner::spinner(format!("{}{}{}{}", "Connecting to master server: ".blue().bold(), masters.ip, " | Name: ", masters.name).parse().expect("spinner working"));
        let ssh_session = ssh::connect_server_via_ssh(&masters);
        spinner_handle.done();
        let ip = &masters.ip;

        for (i, instructions) in installation::get_installation().linux_amd64.iter().enumerate() {
            install::install_common(instructions, &ssh_session);
        }

        if master_node_index == 0 {
            let mut k3s_flag = String::new();
            k3s_flag.push_str("curl -sfL https://get.k3s.io |");
            k3s_flag.push_str(" INSTALL_K3S_VERSION=");
            k3s_flag.push_str(k3s_version);
            k3s_flag.push_str(" sh -s - server --cluster-init");
            let command =k3s_flag;
            install::install_rancher(&ssh_session, command.parse().unwrap(), "Rancher master");
            install::get_kube_config_into_local(ip, &ssh_session);
            install::get_k3s_token_and_save( &ssh_session);
        }

    }
}

fn build_nodes(masters: &Config, k3s_version: &String) {

    let master_k3s_server = &masters.masters[0].ip.replace(":22", ":6443");
    for (master_node_index,masters) in masters.nodes.iter().enumerate() {
        let spinner_handle = spinner::spinner(format!("{}{}{}{}", "Connecting to node server: ".blue().bold(), masters.ip, " | Name: ", masters.name).parse().expect("spinner working"));
        let ssh_session = ssh::connect_server_via_ssh(&masters);
        spinner_handle.done();
        let ip = &masters.ip;

        let mut k3s_flag = String::new();
        k3s_flag.push_str("curl -sfL https://get.k3s.io |");
        k3s_flag.push_str(" INSTALL_K3S_VERSION=");
        k3s_flag.push_str(k3s_version);
        k3s_flag.push_str(" K3S_URL=https://");
        k3s_flag.push_str(master_k3s_server);
        k3s_flag.push_str(" K3S_TOKEN=");
        let mut token = fs::read_to_string("kubeconfig/token").expect("should have been read the file");
        sanitize::strip_trailing_nl(&mut token);
        k3s_flag.push_str(&token);
        k3s_flag.push_str(" sh -");
        let command =k3s_flag;

        install::install_rancher(&ssh_session, command.parse().unwrap(), "Rancher worker");
        for (i, instructions) in installation::get_installation().linux_amd64.iter().enumerate() {
            install::install_common(instructions, &ssh_session);
        }

    }
}


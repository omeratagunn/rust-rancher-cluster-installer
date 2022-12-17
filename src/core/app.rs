use crate::app::yaml::{parse_yaml_config, Config};
use colored::Colorize;
use std::fs;

#[path = "yaml.rs"]
mod config;
#[path = "installation.rs"]
mod install;
#[path = "../config/config.rs"]
mod installation;

#[path = "../utils/spinner.rs"]
mod spinner;
#[path = "../core/ssh.rs"]
mod ssh;
#[path = "yaml.rs"]
mod yaml;

pub(crate) fn app(path: &String, k3s_version: &String) {
    let spinner_handle = spinner::spinner("Parsing yaml file...".parse().expect("spinner working"));

    let parsed_yaml = parse_yaml_config(path);
    spinner_handle.done();

    build_masters(&parsed_yaml, &k3s_version);
    build_nodes(&parsed_yaml, &k3s_version);
}

pub(crate) fn build_masters(masters: &Config, k3s_version: &String) {
    for (master_node_index, masters) in masters.masters.iter().enumerate() {
        let spinner_handle = spinner::spinner(
            format!(
                "{}{}{}{}",
                "Connecting to master server: ".blue().bold(),
                masters.ip,
                " | Name: ",
                masters.name
            )
            .parse()
            .expect("spinner working"),
        );
        let ssh_session = ssh::connect_server_via_ssh(&masters);
        spinner_handle.done();
        let ip = &masters.ip;

        for (_index, instructions) in installation::get_installation()
            .linux_amd64
            .iter()
            .enumerate()
        {
            install::install_common(instructions, &ssh_session);
        }

        if master_node_index == 0 {
            install::install_k3s(
                &ssh_session,
                rancherinstaller::build_k3s_master_command(&k3s_version)
                    .parse()
                    .unwrap(),
                "Rancher master",
            );
            install::get_kube_config_into_local(ip, &ssh_session);
            install::get_k3s_token_and_save(&ssh_session);
        }
    }
}

fn build_nodes(nodes: &Config, k3s_version: &String) {
    for (_index, nodes) in nodes.nodes.iter().enumerate() {
        let spinner_handle = spinner::spinner(
            format!(
                "{}{}{}{}",
                "Connecting to node server: ".blue().bold(),
                nodes.ip,
                " | Name: ",
                nodes.name
            )
            .parse()
            .expect("spinner working"),
        );
        let ssh_session = ssh::connect_server_via_ssh(&nodes);
        spinner_handle.done();
        let mut token =
            fs::read_to_string("kubeconfig/token").expect("should have been read the file");
        install::install_k3s(
            &ssh_session,
            rancherinstaller::build_k3s_node_command(&k3s_version, &nodes.ip, token)
                .parse()
                .unwrap(),
            "Rancher worker",
        );
        for (_i, instructions) in installation::get_installation()
            .linux_amd64
            .iter()
            .enumerate()
        {
            install::install_common(instructions, &ssh_session);
        }
    }
}

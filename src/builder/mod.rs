use crate::builder::install::install::{
    get_k3s_token_and_save, get_kube_config_into_local, install_common, install_k3s,
};
use crate::types::{Config, ServerConnector, Spinner};
use crate::utils::get_kube_config_path;
use crate::{config, utils};
use std::fs;

#[path = "../core/installation.rs"]
mod install;

pub fn build_masters(masters: &Config, k3s_version: &String) {
    for (master_node_index, masters) in masters.masters.iter().enumerate() {
        masters.spinner_start();

        let ssh_session = masters.connect();

        masters.spinner_stop();
        let ip = &masters.ip;

        for (_index, instructions) in config::get_installation().linux_amd64.iter().enumerate() {
            install_common(instructions, &ssh_session);
        }

        if master_node_index == 0 {
            install_k3s(
                &ssh_session,
                utils::build_k3s_master_command(&k3s_version)
                    .parse()
                    .unwrap(),
                "k3s master",
            );
            get_kube_config_into_local(ip, &ssh_session);
            get_k3s_token_and_save(&ssh_session);
        }
    }
}

pub fn build_nodes(nodes: &Config, k3s_version: &String) {
    let masterip = &nodes.masters[0].ip;

    for (_index, nodes) in nodes.nodes.iter().enumerate() {
        nodes.spinner_start();
        let ssh_session = nodes.connect();
        nodes.spinner_stop();

        let token = fs::read_to_string(get_kube_config_path("/token".to_string()))
            .expect("should have been read the file");
        install_k3s(
            &ssh_session,
            utils::build_k3s_node_command(&k3s_version, &masterip, token)
                .parse()
                .unwrap(),
            "k3s worker",
        );
        for (_i, instructions) in config::get_installation().linux_amd64.iter().enumerate() {
            install_common(instructions, &ssh_session);
        }
    }
}

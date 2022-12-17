use colored::Colorize;
use rancherinstaller::builder::{build_masters, build_nodes};
use rancherinstaller::types::Config;
use rancherinstaller::{ssh, utils};

pub(crate) fn app(path: &String, k3s_version: &String, should_delete: bool) {
    let spinner_handle =
        rancherinstaller::utils::spinner("Parsing yaml file...".parse().expect("spinner working"));

    let parsed_yaml = rancherinstaller::yaml::parse_yaml_config(path);
    spinner_handle.done();

    if should_delete {
        delete_k3s(&parsed_yaml);
        return;
    }

    build_masters(&parsed_yaml, &k3s_version);
    build_nodes(&parsed_yaml, &k3s_version);
}

fn delete_k3s(servers: &Config) {
    for (_master_node_index, masters) in servers.masters.iter().enumerate() {
        let spinner_handle = utils::spinner(
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
        let mut command = ssh_session.channel_session().expect("session");
        command
            .exec("/usr/local/bin/k3s-uninstall.sh")
            .expect(&format!("{} Uninstallation", "k3s master"));

        command.wait_close().ok();

        spinner_handle.done();
    }

    for (_master_node_index, nodes) in servers.nodes.iter().enumerate() {
        let spinner_handle = utils::spinner(
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
        let mut command = ssh_session.channel_session().expect("session");
        command
            .exec("/usr/local/bin/k3s-agent-uninstall.sh")
            .expect(&format!("{} Uninstallation", "k3s nodes"));

        command.wait_close().ok();
        spinner_handle.done();
    }
}

use rancherinstaller::builder::{build_masters, build_nodes};
use rancherinstaller::types::{Config, ServerConnector, Spinner};

pub(crate) fn app(path: &String, k3s_version: &String, should_delete: &bool) {
    let spinner_handle =
        rancherinstaller::utils::spinner("Parsing yaml file...".parse().expect("spinner working"));

    let parsed_yaml = rancherinstaller::yaml::parse_yaml_config(path);

    spinner_handle.done();


    if *should_delete {
        delete_k3s(&parsed_yaml);
        return;
    }

    build_masters(&parsed_yaml, &k3s_version);
    build_nodes(&parsed_yaml, &k3s_version);
}

fn delete_k3s(servers: &Config) {
    for (_master_node_index, masters) in servers.masters.iter().enumerate() {
        masters.spinner_start();

        let mut command = masters
            .connect()
            .channel_session()
            .expect("session to work");

        command
            .exec("/usr/local/bin/k3s-uninstall.sh")
            .expect(&format!("{} Uninstallation", "k3s master"));

        command.wait_close().ok();

        masters.spinner_stop()
    }

    for (_node_index, nodes) in servers.nodes.iter().enumerate() {
        nodes.spinner_start();

        let mut command = nodes
            .connect()
            .channel_session()
            .expect("session not to fail");
        command
            .exec("/usr/local/bin/k3s-agent-uninstall.sh")
            .expect(&format!("{} Uninstallation", "k3s nodes"));

        command.wait_close().ok();

        nodes.spinner_stop();
    }
}

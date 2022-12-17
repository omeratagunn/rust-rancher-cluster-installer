use rancherinstaller::builder::{build_masters, build_nodes};


pub(crate) fn app(path: &String, k3s_version: &String) {
    let spinner_handle =
        rancherinstaller::utils::spinner("Parsing yaml file...".parse().expect("spinner working"));

    let parsed_yaml = rancherinstaller::yaml::parse_yaml_config(path);
    spinner_handle.done();

    build_masters(&parsed_yaml, &k3s_version);
    build_nodes(&parsed_yaml, &k3s_version);
}

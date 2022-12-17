use rancherinstaller;
use rancherinstaller::types::{Config, ServerConf};
use std::path::Path;
use std::{env, path};

#[test]
fn test_k3s_master_concat() {
    let res = rancherinstaller::utils::build_k3s_master_command(&"v1.25.4+k3s1".to_string());

    assert_eq!(res, "curl -sfL https://get.k3s.io | INSTALL_K3S_VERSION=v1.25.4+k3s1 sh -s - server --cluster-init".to_string());
}
#[test]
fn test_k3s_node_concat() {
    let res = rancherinstaller::utils::build_k3s_node_command(
        &"v1.25.4+k3s1".to_string(),
        &"182.344.12.12".to_string(),
        "VERYGOODTOKEN".to_string(),
    );
    assert_eq!(res, "curl -sfL https://get.k3s.io | INSTALL_K3S_VERSION=v1.25.4+k3s1 K3S_URL=https://182.344.12.12 K3S_TOKEN=VERYGOODTOKEN sh -".to_string());
}

#[test]
fn test_strip_trailing() {
    let mut test_str = "yodel hej\n".to_string();
    rancherinstaller::utils::strip_trailing_nl(&mut test_str);

    assert_eq!(test_str, "yodel hej");
}

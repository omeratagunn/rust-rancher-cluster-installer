use std::env;
use std::path::Path;
use std::time::Instant;
use terminal_spinners::{SpinnerBuilder, SpinnerHandle, DOTS};

pub fn build_k3s_master_command(version: &String) -> String {
    let mut k3s_flag = String::new();
    k3s_flag.push_str("curl -sfL https://get.k3s.io |");
    k3s_flag.push_str(" INSTALL_K3S_VERSION=");
    k3s_flag.push_str(&version);
    k3s_flag.push_str(" sh -s - server --cluster-init");
    return k3s_flag;
}

pub fn build_k3s_node_command(version: &String, ip: &String, mut token: String) -> String {
    let mut k3s_flag = String::new();
    k3s_flag.push_str("curl -sfL https://get.k3s.io |");
    k3s_flag.push_str(" INSTALL_K3S_VERSION=");
    k3s_flag.push_str(&version);
    k3s_flag.push_str(" K3S_URL=https://");
    k3s_flag.push_str(&ip.replace(":22", ":6443"));
    k3s_flag.push_str(" K3S_TOKEN=");

    strip_trailing_nl(&mut token);
    k3s_flag.push_str(&token);
    k3s_flag.push_str(" sh -");
    return k3s_flag;
}

pub fn start_time() -> Instant {
    Instant::now()
}
pub fn finish_time(start_time: Instant) -> u32 {
    start_time.elapsed().as_secs() as u32
}

pub fn strip_trailing_nl(input: &mut String) {
    let new_len = input
        .char_indices()
        .rev()
        .find(|(_, c)| !matches!(c, '\n' | '\r'))
        .map_or(0, |(i, _)| i + 1);
    if new_len != input.len() {
        input.truncate(new_len);
    }
}

pub fn spinner(text: String) -> SpinnerHandle {
    let handle = SpinnerBuilder::new().spinner(&DOTS).text(text).start();
    return handle;
}

pub fn get_kube_config_path(join: String) -> String {
    let mut kubeconfig_path = String::from("kubeconfig");
    kubeconfig_path.push_str(&join);
    let path = Path::join(
        &env::current_dir().unwrap().as_path(),
        Path::new(&kubeconfig_path).to_str().unwrap(),
    )
    .as_os_str()
    .to_os_string()
    .into_string()
    .expect("well, do not expect much shit goes bananas after all");

    return path;
}

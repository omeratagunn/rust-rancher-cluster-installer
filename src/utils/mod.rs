use std::env;
use std::path::Path;
use std::time::Instant;
use terminal_spinners::{SpinnerBuilder, SpinnerHandle, DOTS};

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
    
    SpinnerBuilder::new().spinner(&DOTS).text(text).start()
}

pub fn get_kube_config_path(join: String) -> String {
    let mut kubeconfig_path = String::from("kubeconfig");
    kubeconfig_path.push_str(&join);
    let path = Path::join(
        env::current_dir().unwrap().as_path(),
        Path::new(&kubeconfig_path).to_str().unwrap(),
    )
    .as_os_str()
    .to_os_string()
    .into_string()
    .expect("well, do not expect much shit goes bananas after all");

    path
}

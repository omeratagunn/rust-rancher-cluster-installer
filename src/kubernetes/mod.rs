pub mod install {
    use crate::utils;
    use crate::utils::get_kube_config_path;
    use ssh2::Session;
    use std::fs::{self, File};
    use std::io::prelude::*;
    use std::path::Path;

    pub fn get_kube_config_into_local(ip: &str, session: &Session) {
        let spinner_handle = utils::spinner(
            "Fetching kube config into local..."
                .parse()
                .expect("spinner working"),
        );

        let mut command = session.channel_session().expect("session");

        command
            .exec("cat /etc/rancher/k3s/k3s.yaml")
            .expect("Fetching kube config");

        let mut s = String::new();

        command.read_to_string(&mut s).expect("Command to run");
        let folder: bool = Path::new(&get_kube_config_path("".to_string())).is_dir();

        if !folder {
            fs::create_dir(get_kube_config_path("".to_string())).expect("creating folder");
        }
        let replaced_config = s.replace(
            "https://127.0.0.1:6443",
            &format!("{}{}", "https://", ip.replace("22", "6443")),
        );

        let path = Path::new("./kubeconfig/config");
        let display = path.display();

        let mut file = match File::create(path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        if let Err(why) = file.write_all(replaced_config.as_bytes()) { panic!("couldn't write to {}: {}", display, why) }

        spinner_handle.done();
    }

    pub fn get_k3s_token_and_save(session: &Session) {
        let spinner_handle = utils::spinner(
            "Fetching k3s token into local..."
                .parse()
                .expect("spinner working"),
        );

        let mut command = session.channel_session().expect("session");

        command
            .exec("cat /var/lib/rancher/k3s/server/node-token")
            .expect("Fetching k3s token into local");

        let mut s = String::new();

        command.read_to_string(&mut s).expect("Command to run");
        let folder: bool = Path::new("./kubeconfig").is_dir();

        if !folder {
            fs::create_dir("./kubeconfig").expect("creating folder");
        }

        let path = Path::new("./kubeconfig/token");
        let display = path.display();

        let mut file = match File::create(path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        if let Err(why) = file.write_all(s.as_bytes()) { panic!("couldn't write to {}: {}", display, why) }

        spinner_handle.done();
    }
}

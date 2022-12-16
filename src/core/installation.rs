use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use ssh2::{Session};
use crate::app::installation::LinuxInstructions;
use crate::app::spinner;

pub const fn folder_path() -> &'static str {
    "./kubeconfig"
}

pub(crate) fn install_common(instructions: &LinuxInstructions, session: &Session){
    let mut info_string = String::new();
    info_string.push_str("Installing ");
    info_string.push_str(&*instructions.name);

    let spinner_handle = spinner::spinner(info_string.parse().expect("spinner working"));

    let mut command = session.channel_session().expect("session");

    command.exec(&*instructions.command).expect(&format!("{} INSTALLATION", instructions.name));
    let mut s = String::new();

    command.read_to_string(&mut s).expect("Command to run");

    // if return length after the command is zero, run fallback command. in this case its a installation scenario //
    if s.len() == 0{

        command.wait_close().ok();

        let mut command = session.channel_session().expect("session");
        command.exec(&*instructions.fallback_command).expect(&format!("{} trying to install", instructions.name));
        let mut s = String::new();

        command.read_to_string(&mut s).expect("Command to run");

        command.wait_close().ok();
        spinner_handle.done();
        return;
    }

    if command.exit_status().expect("exit status") > 0 {
        println!("\n Exited with status code: {}", command.exit_status().unwrap());
    }

    command.read_to_string(&mut s).expect("Command to run");
    command.wait_close().ok();
    spinner_handle.done();
}

pub(crate) fn get_kube_config_into_local(ip: &String, session: &Session){
    let spinner_handle = spinner::spinner("Fetching kube config into local...".parse().expect("spinner working"));


    let mut command = session.channel_session().expect("session");

    command.exec("cat /etc/rancher/k3s/k3s.yaml").expect("Fetching kube config");

    let mut s = String::new();

    command.read_to_string(&mut s).expect("Command to run");
    let folder:bool = Path::new(&folder_path()).is_dir();

    if !folder {
        fs::create_dir(&folder_path()).expect("creating folder");
    }
    let replaced_config = s.replace("https://127.0.0.1:6443", &*format!("{}{}", "https://", ip.replace("22", "6443")));

    let path = Path::new("./kubeconfig/config");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(replaced_config.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        _ => (),
    }

    spinner_handle.done();

}

pub(crate) fn get_k3s_token_and_save(session: &Session){
    let spinner_handle = spinner::spinner("Fetching k3s token into local...".parse().expect("spinner working"));


    let mut command = session.channel_session().expect("session");

    command.exec("cat /var/lib/rancher/k3s/server/node-token").expect("Fetching k3s token into local");

    let mut s = String::new();

    command.read_to_string(&mut s).expect("Command to run");
    let folder:bool = Path::new("./kubeconfig").is_dir();

    if !folder {
        fs::create_dir("./kubeconfig").expect("creating folder");
    }

    let path = Path::new("./kubeconfig/token");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(s.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        _ => (),
    }

    spinner_handle.done();

}

pub(crate) fn install_k3s(session: &Session, command_to_execute: String, rancher_type: &str){
    let mut info_string = String::new();
    info_string.push_str(&*rancher_type);


    let spinner_handle = spinner::spinner(info_string.parse().expect("spinner working"));

    let mut command = session.channel_session().expect("session");

    command.exec(&*command_to_execute).expect(&format!("{} INSTALLATION", rancher_type));
    let mut s = String::new();

    command.read_to_string(&mut s).expect("Command to run");

    if command.exit_status().expect("exit status") > 0 {
        println!("\n Exited with status code: {}", command.exit_status().unwrap());
    }

    command.read_to_string(&mut s).expect("Command to run");
    command.wait_close().ok();
    spinner_handle.done();

}

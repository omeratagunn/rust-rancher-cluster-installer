use crate::kubernetes::install::{get_k3s_token_and_save, get_kube_config_into_local};
use crate::utils;
use crate::utils::{get_kube_config_path, strip_trailing_nl};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use ssh2::Session;
use std::fs;
use std::io::Read;
use std::net::TcpStream;
use terminal_spinners::SpinnerHandle;

pub struct App {
    pub config: String,
    pub install: bool,
    pub delete: bool,
}

pub struct OsInstallationSequence {
    pub linux_amd64: Vec<LinuxInstructions>,
}

impl OsInstallationSequence {
    fn run(&self, session: &Session) {
        for (_index, instructions) in self.linux_amd64.iter().enumerate() {
            let mut info_string = String::new();
            info_string.push_str("Installing ");
            info_string.push_str(&instructions.name);

            let spinner_handle = utils::spinner(info_string.parse().expect("spinner working"));

            let mut command = session.channel_session().expect("session");

            command
                .exec(&instructions.command)
                .unwrap_or_else(|_| panic!("{} INSTALLATION", instructions.name));
            let mut s = String::new();

            command.read_to_string(&mut s).expect("Command to run");

            // if return length after the command is zero, run fallback command. in this case its a installation scenario //
            if s.is_empty() {
                command.wait_close().ok();

                let mut command = session.channel_session().expect("session");
                command
                    .exec(&instructions.fallback_command)
                    .unwrap_or_else(|_| panic!("{} trying to install", instructions.name));
                let mut s = String::new();

                command.read_to_string(&mut s).expect("Command to run");

                command.wait_close().ok();
                spinner_handle.done();
                return;
            }

            if command.exit_status().expect("exit status") > 0 {
                println!(
                    "\n Exited with status code: {}",
                    command.exit_status().unwrap()
                );
            }

            command.read_to_string(&mut s).expect("Command to run");
            command.wait_close().ok();
            spinner_handle.done();
        }
    }
}

pub struct LinuxInstructions {
    pub name: String,
    pub command: String,
    pub fallback_command: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub masters: Vec<ServerConf>,
    pub nodes: Vec<ServerConf>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConf {
    pub name: String,
    pub ip: String,
    pub username: String,
    pub password: String,
    pub k3s_version: String,
}

pub trait ServerConnector {
    fn connect(&self) -> Session;
}

impl ServerConnector for ServerConf {
    fn connect(&self) -> Session {
        let tcp = TcpStream::connect(&self.ip).expect("connection failed");
        let mut sess = Session::new().expect("session failed");
        sess.set_tcp_stream(tcp);
        sess.handshake().expect("handshake failed");
        sess.userauth_password(&self.username, &self.password)
            .expect("userauthgone");

        sess
    }
}

pub trait Spinner {
    fn spinner_start(&self) -> SpinnerHandle;
    fn spinner_stop(&self);
}

impl Spinner for ServerConf {
    fn spinner_start(&self) -> SpinnerHandle {
        return utils::spinner(
            format!(
                "{}{}{}{}",
                "Connecting to server: ".blue().bold(),
                self.ip,
                " | Name: ",
                self.name
            )
            .parse()
            .expect("spinner working"),
        );
    }
    fn spinner_stop(&self) {
        self.spinner_start().done()
    }
}

pub struct ClusterBuilder {
    pub config: Config,
    pub installation: OsInstallationSequence,
    // TODO figure to expect Clusterbuild trait to be implemented for kube_type
    pub kube_type: K3s,
}

pub trait ClusterBuild {
    fn build(&self) -> Result<String, String>;
    fn delete(&self) -> Result<String, String>;
}

impl ClusterBuild for ClusterBuilder {
    fn build(&self) -> Result<String, String> {
        self.build_master();
        self.build_nodes();
        Ok("Build completed".to_string())
    }
    fn delete(&self) -> Result<String, String> {
        self.delete_master();
        self.delete_nodes();
        Ok("Delete completed".to_string())
    }
}

impl ClusterBuilder {
    fn delete_master(&self) {
        for (_master_node_index, masters) in self.config.masters.iter().enumerate() {
            masters.spinner_start();

            let mut command = masters
                .connect()
                .channel_session()
                .expect("session to work");

            command
                .exec("/usr/local/bin/k3s-uninstall.sh")
                .unwrap_or_else(|_| panic!("{} Uninstallation", "k3s master"));

            command.wait_close().ok();

            masters.spinner_stop()
        }
    }

    fn delete_nodes(&self) {
        for (_node_index, nodes) in self.config.nodes.iter().enumerate() {
            nodes.spinner_start();

            let mut command = nodes
                .connect()
                .channel_session()
                .expect("session not to fail");
            command
                .exec("/usr/local/bin/k3s-agent-uninstall.sh")
                .unwrap_or_else(|_| panic!("{} Uninstallation", "k3s nodes"));

            command.wait_close().ok();

            nodes.spinner_stop();
        }
    }

    fn build_master(&self) {
        for (master_node_index, masters) in self.config.masters.iter().enumerate() {
            masters.spinner_start();

            let ssh_session = masters.connect();

            masters.spinner_stop();
            let ip = &masters.ip;
            self.installation.run(&ssh_session);

            if master_node_index == 0 {
                self.kube_type.install(
                    &ssh_session,
                    &String::from("k3s master"),
                    self.kube_type
                        .build_master_command(&masters.k3s_version)
                        .parse()
                        .unwrap(),
                );

                get_kube_config_into_local(ip, &ssh_session);
                get_k3s_token_and_save(&ssh_session);
            }
        }
    }
    fn build_nodes(&self) {
        let masterip = &self.config.masters[0].ip;

        for (_index, nodes) in self.config.nodes.iter().enumerate() {
            nodes.spinner_start();
            let ssh_session = nodes.connect();
            nodes.spinner_stop();

            let token = fs::read_to_string(get_kube_config_path("/token".to_string()))
                .expect("should have been read the file");
            self.kube_type.install(
                &ssh_session,
                &String::from("k3s worker"),
                self.kube_type
                    .build_node_command(&nodes.k3s_version, masterip, token),
            );

            self.installation.run(&ssh_session);
        }
    }
}

pub struct K3s {}

pub trait KubernetesBuilder {
    fn install(&self, session: &Session, cluster_type: &str, command_to_execute: String);
    fn build_master_command(&self, version: &str) -> String;
    fn build_node_command(&self, version: &str, ip: &str, token: String) -> String;
}

impl KubernetesBuilder for K3s {
    fn install(&self, session: &Session, cluster_type: &str, command_to_execute: String) {
        let mut info_string = String::new();
        info_string.push_str(cluster_type);
        let spinner_handle = utils::spinner(info_string.parse().expect("spinner working"));

        let mut command = session.channel_session().expect("session");

        command
            .exec(&command_to_execute)
            .unwrap_or_else(|_| panic!("{} INSTALLATION", cluster_type));
        let mut s = String::new();

        command.read_to_string(&mut s).expect("Command to run");

        if command.exit_status().expect("exit status") > 0 {
            println!(
                "\n Exited with status code: {}",
                command.exit_status().unwrap()
            );
        }

        command.read_to_string(&mut s).expect("Command to run");
        command.wait_close().ok();
        spinner_handle.done();
    }
    fn build_master_command(&self, version: &str) -> String {
        let mut k3s_flag = String::new();
        k3s_flag.push_str("curl -sfL https://get.k3s.io | INSTALL_K3S_VERSION=");
        k3s_flag.push_str(version);
        k3s_flag.push_str(" sh -s - server --cluster-init");
        k3s_flag
    }
    fn build_node_command(&self, version: &str, ip: &str, mut token: String) -> String {
        let mut k3s_flag = String::new();
        k3s_flag.push_str("curl -sfL https://get.k3s.io | INSTALL_K3S_VERSION=");
        k3s_flag.push_str(version);
        k3s_flag.push_str(" K3S_URL=https://");
        k3s_flag.push_str(&ip.replace(":22", ":6443"));
        k3s_flag.push_str(" K3S_TOKEN=");

        strip_trailing_nl(&mut token);
        k3s_flag.push_str(&token);
        k3s_flag.push_str(" sh -");
        k3s_flag
    }
}

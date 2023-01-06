use std::fs;
use crate::{config, utils};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use ssh2::Session;
use std::net::TcpStream;
use terminal_spinners::SpinnerHandle;
use crate::kubernetes::install::{get_k3s_token_and_save, get_kube_config_into_local, install_common, install_k3s};
use crate::utils::get_kube_config_path;

pub struct Installation {
    pub linux_amd64: Vec<LinuxInstructions>,
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
    pub k3s_version: String
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

        return sess;
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

pub struct ClusterBuilder{
    pub config: Config
}

pub trait ClusterBuild{
    fn build(&self) -> Result<String,String>;
    fn delete(&self) -> Result<String,String>;
}

impl ClusterBuild for ClusterBuilder{
    fn build(&self) -> Result<String, String>{
        self.build_master();
        self.build_nodes();
        Ok("Build completed".to_string())
    }
    fn delete(&self) -> Result<String, String>{
        self.delete_master();
        self.delete_nodes();
        Ok("Delete completed".to_string())
    }
}

impl ClusterBuilder{
    fn delete_master(&self){
        for (_master_node_index, masters) in self.config.masters.iter().enumerate() {
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
    }

    fn delete_nodes(&self){
        for (_node_index, nodes) in self.config.nodes.iter().enumerate() {
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

    fn build_master(&self){
        for (master_node_index, masters) in self.config.masters.iter().enumerate() {
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
                    utils::build_k3s_master_command(&masters.k3s_version)
                        .parse()
                        .unwrap(),
                    "k3s master",
                );

                get_kube_config_into_local(ip, &ssh_session);
                get_k3s_token_and_save(&ssh_session);
            }
        }
    }
    fn build_nodes(&self){
        let masterip = &self.config.masters[0].ip;

        for (_index, nodes) in self.config.nodes.iter().enumerate() {
            nodes.spinner_start();
            let ssh_session = nodes.connect();
            nodes.spinner_stop();

            let token = fs::read_to_string(get_kube_config_path("/token".to_string()))
                .expect("should have been read the file");
            install_k3s(
                &ssh_session,
                utils::build_k3s_node_command(&nodes.k3s_version, &masterip, token)
                    .parse()
                    .unwrap(),
                "k3s worker",
            );
            for (_i, instructions) in config::get_installation().linux_amd64.iter().enumerate() {
                install_common(instructions, &ssh_session);
            }
        }
    }
}

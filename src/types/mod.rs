use crate::utils;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use ssh2::Session;
use std::net::TcpStream;
use terminal_spinners::SpinnerHandle;

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

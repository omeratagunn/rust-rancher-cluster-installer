use serde::{Deserialize, Serialize};

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

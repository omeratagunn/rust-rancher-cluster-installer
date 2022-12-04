use std::io::prelude::*;
use colored::Colorize;
use ssh2::{Channel, Session};
use crate::app::spinner;

pub(crate) fn check_curl(channel: &Session){
    let spinner_handle = spinner::spinner(" Installing curl...".parse().expect("spinner working"));
    let mut curl = channel.channel_session().expect("session");
    curl.exec("curl --version").expect("curl installation");
    let mut s = String::new();

    curl.read_to_string(&mut s).expect("Command to run");

    if s.len() == 0 {

        curl.wait_close().ok();
        install_curl(&channel);
        return;
    }

    curl.read_to_string(&mut s).expect("Command to run");
    curl.wait_close().ok();
    spinner_handle.done();
}

pub(crate) fn install_curl(session: &Session){
    let mut curl = session.channel_session().expect("session");
    curl.exec("apt install curl -y").expect("curl install");
    let mut s = String::new();

    curl.read_to_string(&mut s).expect("Command to run");

    curl.wait_close().ok();
}

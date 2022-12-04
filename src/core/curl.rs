use std::io::prelude::*;
use colored::Colorize;
use ssh2::{Channel, Session};

pub(crate) fn check_curl(channel: &Session){
    let mut curl = channel.channel_session().expect("session");
    curl.exec("curl --version").expect("curl installation");
    let mut s = String::new();

    curl.read_to_string(&mut s).expect("Command to run");

    if s.len() == 0 {
        println!("{}","\nCurl is not installed.. ".red());

        curl.wait_close().ok();
        println!("{}","\nInstalling curl.. ".red());
        install_curl(&channel);
        return;
    }

    curl.read_to_string(&mut s).expect("Command to run");
    println!("{}","\nCurl installed, moving next.. ".green());
    curl.wait_close().ok();
}

pub(crate) fn install_curl(session: &Session){
    let mut curl = session.channel_session().expect("session");
    curl.exec("apt install curl -y").expect("curl install");
    let mut s = String::new();

    curl.read_to_string(&mut s).expect("Command to run");

    curl.wait_close().ok();
}

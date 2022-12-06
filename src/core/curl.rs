use std::io::prelude::*;
use colored::Colorize;
use ssh2::{Channel, Session};
use crate::app::installation::LinuxInstructions;
use crate::app::spinner;

pub(crate) fn install(instructions: LinuxInstructions, session: &Session){
    let mut info_string = String::new();
    info_string.push_str("Installing ");
    info_string.push_str(&*instructions.name);

    let spinner_handle = spinner::spinner(info_string.parse().expect("spinner working"));

    let mut command = session.channel_session().expect("session");

    command.exec(&*instructions.command).expect(&format!("{} INSTALLATION", instructions.name));
    let mut s = String::new();

    command.read_to_string(&mut s).expect("Command to run");


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

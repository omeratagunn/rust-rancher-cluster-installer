use ssh2::{Channel, Session};
use std::io::prelude::*;
use std::net::TcpStream;
use colored::Colorize;
use crate::app::types::SshCredArgs;


pub(crate) fn connect_server_via_ssh(args: &SshCredArgs) -> Session {
    let tcp = TcpStream::connect(&args.cred.ip).expect("connection failed");
    let mut sess = Session::new().expect("session failed");
    sess.set_tcp_stream(tcp);
    sess.handshake().expect("handshake failed");
    sess.userauth_password(&args.cred.username, &args.cred.password)
        .expect("userauthgone");

    return sess;
}



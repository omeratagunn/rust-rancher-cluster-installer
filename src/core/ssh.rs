use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;
use crate::app::types::SshCredArgs;


pub(crate) fn connect_server_via_ssh(args: &SshCredArgs) {
    let tcp = TcpStream::connect(&args.cred.ip).expect("connection failed");
    let mut sess = Session::new().expect("session failed");
    sess.set_tcp_stream(tcp);
    sess.handshake().expect("handshake failed");
    sess.userauth_password(&args.cred.username, &args.cred.password)
        .expect("userauthgone");
    let mut channel = sess.channel_session().unwrap();
    channel.exec("show version").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close().ok();
    println!("{}", channel.exit_status().unwrap());
}

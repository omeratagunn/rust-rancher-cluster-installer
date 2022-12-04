use ssh2::{Channel, Session};
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
    let mut channel = sess.channel_session().expect("session failed");
    let mut session = install_k3s(channel);
}
//curl -sfL https://get.k3s.io | INSTALL_K3S_VERSION="***" sh -s - server --cluster-init

fn install_k3s(channel: Channel) -> Channel{
    let mut install = channel;
    install.exec("ls -la").unwrap();
    let mut s = String::new();
    install.read_to_string(&mut s).expect("Command to run");
    println!("{}", s);

    install.wait_close().ok();
    println!("{}", install.exit_status().expect("Closing session"));
    return install
}

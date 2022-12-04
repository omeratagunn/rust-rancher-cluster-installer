use std::io::Read;
use ssh2::{Channel, Session};

pub(crate) fn install_longhorn_reqs(session: &Session){
    let mut curl = session.channel_session().expect("session");
    curl.exec("jq --version").expect("jq install");

    let mut s = String::new();

    curl.read_to_string(&mut s).expect("Command to run");
    if s.len() == 0 {
        install_jq(session);
        curl.wait_close().ok();
    }
    curl.wait_close().ok();

}

fn install_jq(session: &Session){
    let mut curl = session.channel_session().expect("session");
    curl.exec("apt-get update && apt-get install jq -y").expect("jq install");

    let mut s = String::new();

    curl.read_to_string(&mut s).expect("Command to run");

    curl.wait_close().ok();
}

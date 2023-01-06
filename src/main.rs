#[path = "core/app.rs"]
mod app;

use crate::app::app;

use std::env;
use rancherinstaller::input::{help, match_args};

fn main() {
    let start = rancherinstaller::utils::start_time();
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        help();
        return;
    }
    let parsed_args = match_args(args);


    app(
        &parsed_args.path,
        &parsed_args.k3s_version,
        &parsed_args.should_delete,
    );

    println!(
        "Time taken for installation: {} seconds",
        rancherinstaller::utils::finish_time(start)
    )
}


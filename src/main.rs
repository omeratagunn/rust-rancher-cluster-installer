#[path = "core/app.rs"]
mod app;

use crate::app::app;
use colored::Colorize;

use std::env;

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
        parsed_args.should_delete,
    );

    println!(
        "Time taken for installation: {} seconds",
        rancherinstaller::utils::finish_time(start)
    )
}

struct MatchArgsReturnType {
    path: String,
    k3s_version: String,
    should_delete: bool,
}

fn match_args(args: Vec<String>) -> MatchArgsReturnType {
    let mut path = String::new();
    let mut k3s_version = String::new();
    let mut should_delete = false;
    for (i, arg) in args.iter().enumerate() {
        if arg == "help" {
            help();
            break;
        }
        if arg == "config" && args[i + 1].contains("/") {
            path.push_str(&args[i + 1]);
        }
        if arg == "k3s_version" && args[i + 1].contains("v") {
            k3s_version.push_str(&args[i + 1]);
        }
        if arg == "delete" {
            should_delete = true
        }
    }
    if k3s_version.len() == 0 {
        k3s_version.push_str("v1.23.13+k3s1")
    }

    return MatchArgsReturnType {
        k3s_version,
        should_delete,
        path,
    };
}

fn help() {
    println!("{}", "- Example usage: ./rancherinstall -- config <absoluteyamlfilepath>\n- Optionally you can pass k3s version -- k3s_version v1.23.13+k3s1".blue().bold());
    println!("{}", "- Example usage for deleting installed k3s clusters: ./rancherinstall -- config <absoluteyamlfilepath> -- delete".blue().bold())
}

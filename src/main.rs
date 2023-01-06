use clap::Parser;
use rancherinstaller::app::app;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Absolute path to yaml file to connect and setup...
    #[arg(short, long)]
    servers: String,
    /// Required with --servers to delete k3s installation
    #[arg(short, long, default_value_t = false)]
    delete: bool,
}
fn main() {
    let args = Args::parse();
    let start = rancherinstaller::utils::start_time();

    app(
        &args.servers,
        &args.delete,
    );

    println!(
        "Time taken for installation: {} seconds",
        rancherinstaller::utils::finish_time(start)
    )
}


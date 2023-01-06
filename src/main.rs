use clap::Parser;
use rancherinstaller::app::app;
use rancherinstaller::types::App;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Absolute path to yaml file to connect and setup...
    #[arg(short, long)]
    config: String,
    /// Required with --config to install k3s into given servers...
    #[arg(short, long, default_value_t = false)]
    install: bool,
    /// Required with --config to delete k3s installation...
    #[arg(short, long, default_value_t = false)]
    delete: bool,
}

fn main() {
    let args = Args::parse();
    let start = rancherinstaller::utils::start_time();
    let app_config = App{
        config: args.config,
        install: args.install,
        delete: args.delete
    };
    app(&app_config);

    println!(
        "Time taken for installation: {} seconds",
        rancherinstaller::utils::finish_time(start)
    )
}

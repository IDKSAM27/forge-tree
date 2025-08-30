use clap::Command;
use colored::*;
use forge_tree::cli::Cli;

fn main() {
    let app = Cli::new();
    let matches = app.get_matches();

    if let Err(e) = Cli::run(matches) {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}

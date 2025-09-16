mod cli;
mod models;

use clap::Parser;
use cli::commands::{Cli, execute_command};
use std::process;

fn main() {
    let cli = Cli::parse();
    
    if let Err(e) = execute_command(cli) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

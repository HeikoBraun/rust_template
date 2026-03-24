use crate::main_functions::print_about;
use clap::Parser;
use env_logger::Env;
use log::debug;
use std::process::exit;

mod cli;
mod main_functions;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cli_args = cli::Cli::parse();

    if cli_args.about {
        print_about();
        exit(0);
    }
    debug!("cli_args: {:?}", cli_args);

    // Main program logic
    println!("Hello, world!");
}

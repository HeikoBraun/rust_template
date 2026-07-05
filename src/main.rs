use crate::about_functions::print_about;
use clap::{CommandFactory, Parser};
use env_logger::Env;
use log::debug;

mod about_functions;
mod cli;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cli_args = cli::Cli::parse();

    if cli_args.help_all {
        // Build a fresh command and un-hide all args
        let mut cmd = cli::Cli::command();

        // Unhide everything
        for id in cmd
            .get_arguments()
            .map(|a| a.get_id().clone())
            .collect::<Vec<_>>()
        {
            cmd = cmd.mut_arg(id, |a| a.hide(false));
        }
        cmd.print_help().unwrap();
        return;
    }

    if cli_args.about {
        print_about();
        return;
    }
    debug!("cli_args: {:?}", cli_args);

    // Main program logic
    println!("Hello, world!");
}

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about="<ToDo: description of program>", long_about = None)]
pub struct Cli {
    /// filename
    #[arg(help = "filename to be processed", required_unless_present_any = ["about","help_all"])]
    pub filename: Option<String>,

    /// toml
    #[arg(short = 't', long = "toml", help = "Toml file for config.")]
    pub toml_file: Option<String>,

    /// dry_run
    #[arg(
        short = 'n',
        long = "dry-run",
        default_value_t = false,
        help = "dry run, no commands will be executed"
    )]
    pub dry_run: bool,

    /// about
    #[arg(
        long = "about",
        default_value_t = false,
        help = "If set, show details of this programs build and exit."
    )]
    pub about: bool,

    /// hidden option
    #[arg(
        short,
        long,
        hide = true,
        default_value_t = 1,
        help = "Runs in parallel, 0 = num of cores, 1 = no parallelism"
    )]
    pub parallel: u8,

    /// Show full help including hidden options
    #[arg(long = "help-all", default_value_t = false, help = "Show all options")]
    pub help_all: bool,
}

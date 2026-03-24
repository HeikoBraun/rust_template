use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about="<ToDo: description of program>", long_about = None)]
pub struct Cli {
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
        help = "More details about this program"
    )]
    pub about: bool,
}

use log::{debug, error, info};
use std::fs;
use std::process::{exit, Command};
use toml::Table;

fn read_file(filename: &str, ignore_not_exists: bool) -> String {
    match fs::read_to_string(filename) {
        Ok(contents) => {
            debug!("Read file '{}'", filename);
            contents
        }
        Err(e) => {
            if ignore_not_exists {
                info!("File does not exist: '{}'", filename);
                String::new()
            } else {
                error!("Error reading file '{}': {}", filename, e);
                exit(1)
            }
        }
    }
}

pub fn read_toml(filename: &str, ignore_not_exists: bool) -> Table {
    let contents = read_file(filename, ignore_not_exists);
    if contents.is_empty() {
        Table::new()
    } else {
        contents.parse::<Table>().unwrap_or_else(|err| {
            error!("{}", err);
            exit(1)
        })
    }
}

pub fn run_cmd(mut cmd: Command, dry_run: bool) -> (u8, String) {
    if dry_run {
        let program_str = cmd.get_program().to_string_lossy();
        let args_str = cmd
            .get_args()
            .map(|arg| arg.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ");
        info!("Dry-Run: {program_str} {args_str}");
        return (0, String::new());
    }

    match cmd.output() {
        Ok(output) => {
            let code = output.status.code().unwrap_or(1) as u8;
            let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
            (code, stdout)
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("       cmd was:   {:?}", cmd.get_program());
            eprintln!("       args were: {:?}", cmd.get_args());
            eprintln!(
                "       cwd:       {:?}",
                cmd.get_current_dir().unwrap_or("./".as_ref())
            );
            exit(1);
        }
    }
}

use log::{debug, error, info};
use std::io::Read;
use std::process::{exit, Command, Stdio};
use std::time::{Duration, Instant};
use std::{fs, io, thread};
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

#[derive(Debug)]
pub enum RunCmdError {
    Spawn(io::Error),
    Timeout(u64),
    Wait(io::Error),
    StdoutRead(io::Error),
    Utf8(std::string::FromUtf8Error),
}

pub fn run_cmd_with_timeout(
    mut cmd: Command,
    dry_run: bool,
    timeout_ms: u64,
) -> Result<(u8, String), RunCmdError> {
    if dry_run {
        let program_str = cmd.get_program().to_string_lossy();
        let args_str = cmd
            .get_args()
            .map(|arg| arg.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ");
        info!("Dry-Run: {program_str} {args_str}");
        return Ok((0, String::new()));
    }

    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::null()); // or Stdio::piped() if you also want stderr

    let mut child = cmd.spawn().map_err(RunCmdError::Spawn)?;

    let stdout = child.stdout.take();

    let deadline = Instant::now() + Duration::from_millis(timeout_ms);

    loop {
        match child.try_wait().map_err(RunCmdError::Wait)? {
            Some(status) => {
                let code = status.code().unwrap_or(1) as u8;

                let mut stdout_str = String::new();
                if let Some(mut out) = stdout {
                    let mut buf = Vec::new();
                    out.read_to_end(&mut buf).map_err(RunCmdError::StdoutRead)?;
                    stdout_str = String::from_utf8(buf).map_err(RunCmdError::Utf8)?;
                }

                return Ok((code, stdout_str));
            }
            None => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(RunCmdError::Timeout(timeout_ms));
                }
                thread::sleep(Duration::from_millis(10));
            }
        }
    }
}

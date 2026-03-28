use std::{error::Error, process::Command};

fn git_remote_url(remote: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["remote", "get-url", remote])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let text = String::from_utf8(output.stdout).ok()?;
    let url = text.trim();
    if url.is_empty() {
        None
    } else {
        Some(url.to_string())
    }
}

fn git_branch_name() -> Option<String> {
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let text = String::from_utf8(output.stdout).ok()?;
    let txt = text.trim();
    if txt.is_empty() {
        None
    } else {
        Some(txt.to_string())
    }
}

fn git_commit_hash() -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let text = String::from_utf8(output.stdout).ok()?;
    Some(text.trim().to_string())
}

fn deps_via_cargo_tree(kind: &str) -> Result<String, Box<dyn Error>> {
    let pkg_name = std::env::var("CARGO_PKG_NAME")?;

    let output = Command::new("cargo")
        .args([
            "tree", "-e", kind, "-p", &pkg_name, "--prefix", "none", "--depth", "1",
        ])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "cargo tree ({kind}) failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let text = String::from_utf8(output.stdout)?;

    let mut pairs = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if i == 0 {
            continue; // skip the package itself
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split_whitespace();
        let name = match parts.next() {
            Some(v) => v,
            None => continue,
        };
        let ver = match parts.next() {
            Some(v) => v.trim_start_matches('v'),
            None => continue,
        };

        pairs.push(format!("{name}={ver}"));
    }

    pairs.sort();
    Ok(pairs.join(","))
}

fn rustc_version() -> Option<String> {
    let output = Command::new("rustc").args(["--version"]).output().ok()?;

    if !output.status.success() {
        return None;
    }

    let text = String::from_utf8(output.stdout).ok()?;
    Some(text.trim().to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    let normal_deps = deps_via_cargo_tree("normal")?;
    let build_deps = deps_via_cargo_tree("build")?;

    println!("cargo:rustc-env=APP_NORMAL_DEPS={normal_deps}");
    println!("cargo:rustc-env=APP_BUILD_DEPS={build_deps}");

    // Git
    let git_remote_url = git_remote_url("origin").unwrap_or_default();
    let git_branch = git_branch_name().unwrap_or_default();
    let git_commit = git_commit_hash().unwrap_or_default();

    println!("cargo:rustc-env=APP_GIT_REMOTE_URL={git_remote_url}");
    println!("cargo:rustc-env=APP_GIT_BRANCH={git_branch}");
    println!("cargo:rustc-env=APP_GIT_COMMIT={git_commit}");

    let rustc_version = rustc_version().unwrap_or_else(|| "<unknown>".to_string());
    println!("cargo:rustc-env=APP_RUSTC_VERSION={rustc_version}");

    Ok(())
}

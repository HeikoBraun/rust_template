pub fn print_about() {
    println!("Authors: {}", env!("CARGO_PKG_AUTHORS"));
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    // Git repo? -> info!
    if !env!("APP_GIT_COMMIT").is_empty() {
        println!("Git:");
        // has a remote repository, so print all git info
        if !env!("APP_GIT_REMOTE_URL").is_empty() {
            println!("    Remote URL: {}", env!("APP_GIT_REMOTE_URL"));
        }
        println!("    Branch:     {}", env!("APP_GIT_BRANCH"));
        println!("    Commit:     {}", env!("APP_GIT_COMMIT"));
    }

    println!(
        "Compiler: {}",
        option_env!("APP_RUSTC_VERSION").unwrap_or("<unknown>")
    );

    let normal_deps = option_env!("APP_NORMAL_DEPS").unwrap_or("");
    let build_deps = option_env!("APP_BUILD_DEPS").unwrap_or("");

    if !normal_deps.is_empty() {
        println!("Dependencies:");
        _list_dependencies(normal_deps);
    }

    if !build_deps.is_empty() {
        println!("Build dependencies:");
        _list_dependencies(build_deps);
    }
}

fn _list_dependencies(deps_str: &str) {
    for pair in deps_str.split(',').filter(|s| !s.is_empty()) {
        let (name, version) = pair.split_once('=').unwrap_or((pair, "<unknown>"));
        println!("  - {name}: {version}");
    }
}

fn validate_target(args: &[String]) {
    let mut look_for_target: bool = false;
    const VALID: [&str; 3] = [
        "aarch64-apple-ios",
        "aarch64-apple-ios-sim",
        "x86_64-apple-ios",
    ];

    for arg in args.iter() {
        match (look_for_target, arg.as_str()) {
            (_, "--target") => look_for_target = true,
            (true, slice) => {
                if VALID.contains(&slice) {
                    return;
                } else {
                    panic!("Invalid specified IOS target: {}", slice);
                }
            }
            (false, _) => {}
        }
    }

    panic!("xcodegen: --target must be set and must target one of the valid ios targets");
}

fn main() {
    #[cfg(not(target_os = "macos"))]
    {
        panic!("XCode generation only supported on MacOS");
    }

    const CARGO_PATH: &str = env!("CARGO");
    const CARGO_DIR: &str = env!("CARGO_MANIFEST_DIR");
    let workspace_root = std::path::Path::new(&CARGO_DIR)
        .parent()
        .expect("Could not find root workspace");
    let cargo_profile_release_debug: &str = option_env!("CARGO_PROFILE_RELEASE_DEBUG")
        .or(Some("1"))
        .unwrap();

    let args: Vec<String> = std::env::args().skip(1).collect();
    validate_target(&args);
    println!(
        "Executing '{}' in dir '{:?}' with args:",
        CARGO_PATH, workspace_root
    );
    for arg in args.iter() {
        println!("\t{}", arg);
    }

    let mut proc = std::process::Command::new(CARGO_PATH);
    let status = proc
        .current_dir(workspace_root)
        .args(args)
        .env("CARGO_PROFILE_RELEASE_DEBUG", cargo_profile_release_debug);

    match status.status() {
        Ok(ec) => println!("Cargo build exited with code: {}", ec),
        Err(e) => panic!("Cargo build failed with error: {}", e),
    }
}

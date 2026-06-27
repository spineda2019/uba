fn main() -> std::io::Result<()> {
    #[cfg(not(target_os = "macos"))]
    {
        eprintln!("WARNING: Not running on MacOS.");
        eprintln!("It is highly likley this xcode generation will fail");
        eprintln!("or you won't be able to open the generated project.");
        eprintln!("If you promise you understand what you are doing, press enter to continue...");

        let console = std::io::stdin();
        console.read_line(&mut String::new()).unwrap();
    }

    println!("Generating xcode project for uba-ios");

    let cargo_dir: std::path::PathBuf = env!("CARGO_MANIFEST_DIR").into();
    let workspace_root = cargo_dir.parent().expect("could not find workspace root");

    let mut xcodegen_proc = std::process::Command::new("xcodegen");
    let xcodegen_proc = xcodegen_proc
        .current_dir(workspace_root)
        .args(["-s", "uba-entry/uba-ios/project.yml"]);

    match xcodegen_proc.output() {
        Ok(out) => {
            print!("{}", String::from_utf8(out.stdout).unwrap());
        }
        Err(err) => {
            eprintln!("xcodegen: {}", err);
            return Err(err);
        }
    }

    let mut args = std::env::args();
    if args.find(|arg| arg == "open").is_some() {
        println!("Launching xcode...");

        let mut open_proc = std::process::Command::new("open");
        open_proc
            .current_dir(workspace_root)
            .arg("uba-entry/uba-ios/Uba.xcodeproj")
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("could not open Uba.xcodeproj");
    }

    Ok(())
}

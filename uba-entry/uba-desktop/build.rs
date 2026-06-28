fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    const CONSOLE_LOG: &str = "console_log";
    println!("cargo:rustc-check-cfg=cfg({})", CONSOLE_LOG);
    if std::env::var("PROFILE").unwrap() == "debug" {
        println!("cargo:rustc-cfg={}", CONSOLE_LOG);
    }
}

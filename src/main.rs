#![cfg_attr(
    all(not(debug_assertions), not(target_os = "android")),
    windows_subsystem = "windows"
)]

slint::include_modules!();

fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    AppWindow::new()?.run()?;
    Ok(())
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    run_app().unwrap();
}

#[cfg(not(target_os = "android"))]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_app()
}

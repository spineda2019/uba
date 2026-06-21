#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = uba_core::app_name();
    AppWindow::new()?.run()?;
    Ok(())
}

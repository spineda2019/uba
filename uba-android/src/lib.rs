slint::include_modules!();

fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let _ = uba_core::app_name();
    AppWindow::new()?.run()?;
    Ok(())
}

#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    run_app().unwrap();
}

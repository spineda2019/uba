slint::include_modules!();

fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    AppWindow::new()?.run()?;
    Ok(())
}

#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    run_app().unwrap();
}

use uba::launch;
use uba_core::log::Logger;

#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) {
    #[cfg(not(target_os = "android"))]
    {
        compile_error!("uba-android must be built targeting android");
    }

    slint::android::init(app).unwrap();

    let logger = Logger::new(std::io::stderr());

    launch(logger).unwrap();
}

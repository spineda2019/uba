#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use uba_core::log::Logger;

slint::include_modules!();

fn main() -> std::io::Result<()> {
    let mut logger;

    #[cfg(target_os = "ios")]
    {
        logger = Logger::new_transparent();
    }
    #[cfg(not(target_os = "ios"))]
    {
        logger = match Logger::from_file("uba.log") {
            Ok(l) => l,
            Err(e) => panic!("Could not create logger: {}", e),
        };
    }

    logger.log_msg("Starting...")?;

    let window = match AppWindow::new() {
        Ok(app) => app,
        Err(err) => {
            logger.log_error(&err)?;
            return Err(std::io::Error::other(err));
        }
    };

    logger.log_msg("Window constructed")?;

    if let Err(err) = window.run() {
        logger.log_error(&err)?;
        return Err(std::io::Error::other(err));
    }

    Ok(())
}

slint::include_modules!();

pub fn launch(mut logger: uba_core::log::Logger<impl std::io::Write>) -> std::io::Result<()> {
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

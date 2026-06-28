slint::include_modules!();

mod controller;
mod model;

pub fn launch(mut logger: uba_core::log::Logger<impl std::io::Write>) -> std::io::Result<()> {
    logger.log_msg("Starting...")?;

    let app = match controller::MainController::new() {
        Ok(app) => app,
        Err(err) => {
            logger.log_error(&err)?;
            return Err(err);
        }
    };

    app.bind();

    logger.log_msg("View constructed")?;
    logger.log_msg("Initializing view callbacks")?;

    if let Err(err) = app.run() {
        logger.log_error(&err)?;
        return Err(std::io::Error::other(err));
    }

    Ok(())
}

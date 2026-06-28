use uba_core::persistence;

slint::include_modules!();

mod controller;
mod model;

pub fn launch(mut logger: uba_core::log::Logger<impl std::io::Write>) -> std::io::Result<()> {
    logger.log_msg("Starting...")?;

    let app = match controller::MainController::new() {
        Ok(app) => app,
        Err(err) => {
            logger.log_error(&err, None::<&str>)?;
            return Err(err);
        }
    };

    app.bind();

    logger.log_msg("View constructed")?;
    logger.log_msg("Initializing view callbacks")?;

    if let Some(config_dir) = persistence::get_configuration_dir() {
        logger.log_msg(format!("config dir: {:?}", config_dir.as_os_str()))?;
        match std::fs::exists(&config_dir) {
            Ok(exists) => {
                if !exists && let Err(err) = std::fs::create_dir_all(&config_dir) {
                    logger.log_error(err, Some("Could not create directory"))?;
                }
            }
            Err(err) => {
                logger.log_error(&err, None::<&str>)?;
                return Err(err);
            }
        }
    } else {
        logger.log_msg("Could not find config dir")?;
    }

    if let Err(err) = app.run() {
        logger.log_error(&err, None::<&str>)?;
        return Err(std::io::Error::other(err));
    }

    Ok(())
}

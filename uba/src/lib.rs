use uba_core::persistence::{self, Config};

slint::include_modules!();

mod controller;
mod model;

pub fn launch(mut logger: uba_core::log::Logger<impl std::io::Write>) -> std::io::Result<()> {
    logger.log_msg("Starting...")?;

    let mut app = match controller::MainController::new() {
        Ok(app) => app,
        Err(err) => {
            logger.log_error(&err, None::<&str>)?;
            return Err(err);
        }
    };

    // registers casllbacks
    app.bind();

    logger.log_msg("View constructed")?;
    logger.log_msg("Initializing view callbacks")?;

    let conf: Config = if let Some(mut config_dir) = persistence::get_configuration_dir() {
        logger.log_msg(format!("config dir: {:?}", config_dir.as_os_str()))?;
        match std::fs::exists(&config_dir) {
            Ok(exists) => {
                if !exists && let Err(err) = std::fs::create_dir_all(&config_dir) {
                    logger.log_error(err, Some("Could not create directory"))?;
                }

                config_dir.push("uba.toml");

                match Config::from_file(&config_dir) {
                    Ok(c) => c,
                    Err(err) => {
                        logger.log_error(
                            &err,
                            Some(format!(
                                "Could not create config from file: {:?}",
                                &config_dir
                            )),
                        )?;
                        return Err(err);
                    }
                }
            }
            Err(err) => {
                logger.log_error(&err, None::<&str>)?;
                return Err(err);
            }
        }
    } else {
        logger.log_warning("Could not find config dir, using default config.")?;
        Config::default()
    };

    app.load_config(&conf);

    logger.log_msg(format!("Using conf: {}", conf))?;

    if let Err(err) = app.run() {
        logger.log_error(&err, None::<&str>)?;
        return Err(std::io::Error::other(err));
    }

    Ok(())
}

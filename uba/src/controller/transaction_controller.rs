use uba_core::persistence;

use crate::{model::TransactionModel, repository::TransactionRepository};

pub struct TransactionController {
    model: crate::model::TransactionModel,
    repo: crate::repository::TransactionRepository<std::fs::File>,
}

impl TransactionController {
    pub fn new(logger: &mut uba_core::log::Logger<impl std::io::Write>) -> std::io::Result<Self> {
        let mut config_dir: std::path::PathBuf =
            persistence::get_configuration_dir().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::NotFound, "Config dir not found")
            })?;

        if !std::fs::exists(&config_dir)? {
            std::fs::create_dir_all(&config_dir)?;
        }

        config_dir.push("transactions.toml");

        logger.log_msg(format!("Using transaction config: {:?}", &config_dir))?;

        let make_fresh: bool = !std::fs::exists(&config_dir)?;

        let f: std::fs::File = std::fs::OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&config_dir)?;

        let mut repo = TransactionRepository::new(f);

        let model: TransactionModel = if make_fresh {
            logger.log_msg("Creating fresh config")?;
            let model = TransactionModel::default();
            logger.log_msg(format!("Default config: {}", model))?;
            let new_storage = model.serialize().expect("Serialization failed");
            repo.save(&new_storage).expect("Storage commit failed");

            model
        } else {
            logger.log_msg("Loading existing config")?;
            TransactionModel::from_file(repo.borrow_handle())?
        };

        Ok(Self { model, repo })
    }

    pub fn get_balance(&self) -> usize {
        self.model.get_balance()
    }

    pub fn increment_and_get_balance(&mut self) -> usize {
        // TODO(SEP): Repo work
        let new_bal = self.model.increment_and_get_balance();

        self.save();

        new_bal
    }

    fn save(&mut self) {
        let new_storage = self.model.serialize().expect("Serialization failed");
        self.repo.save(&new_storage).expect("Storage commit failed");
    }
}

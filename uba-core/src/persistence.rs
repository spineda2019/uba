pub fn get_configuration_dir() -> Option<std::path::PathBuf> {
    directories_next::ProjectDirs::from("com", "spineda2019", "uba")
        .map(|pd| pd.config_dir().to_owned())
}

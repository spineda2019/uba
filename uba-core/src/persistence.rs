pub fn get_configuration_dir() -> Option<std::path::PathBuf> {
    directories_next::ProjectDirs::from("com", "spineda2019", "uba")
        .map(|pd| pd.config_dir().to_owned())
}

pub trait Truncate: std::io::Seek {
    fn set_len(&self, len: u64) -> std::io::Result<()>;
}

impl Truncate for std::fs::File {
    fn set_len(&self, len: u64) -> std::io::Result<()> {
        std::fs::File::set_len(self, len)
    }
}

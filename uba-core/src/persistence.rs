use std::io::{Read, Write};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
    balance: usize,
}

impl Config {
    pub fn from_file(path: &std::path::Path) -> std::io::Result<Self> {
        let mut f: std::fs::File = {
            if !path.exists() {
                let mut f: std::fs::File = std::fs::File::create_new(path)?;
                let default: Self = Self::default();
                let serialized: String =
                    toml::to_string(&default).map_err(std::io::Error::other)?;
                f.write_all(&serialized.into_bytes())?;
                return Ok(default);
            } else {
                std::fs::File::open(path)?
            }
        };

        let mut buf: String = String::with_capacity(512);

        f.read_to_string(&mut buf)?;

        toml::from_str(&buf).map_err(std::io::Error::other)
    }

    pub fn get_balance(&self) -> usize {
        self.balance
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ balance: {} }}", self.balance)
    }
}

impl std::default::Default for Config {
    #[allow(clippy::derivable_impls)]
    fn default() -> Self {
        Self { balance: 0 }
    }
}

pub fn get_configuration_dir() -> Option<std::path::PathBuf> {
    directories_next::ProjectDirs::from("com", "spineda2019", "uba")
        .map(|pd| pd.config_dir().to_owned())
}

use std::io::{Read, Seek, Write};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
    balance: usize,

    #[serde(skip)]
    file: Option<std::fs::File>,
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
                std::fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(path)?
            }
        };

        let mut buf: String = String::with_capacity(512);

        f.read_to_string(&mut buf)?;

        let mut conf: Self = toml::from_str(&buf).map_err(std::io::Error::other)?;
        conf.file = Some(f);
        Ok(conf)
    }

    pub fn get_balance(&self) -> usize {
        self.balance
    }

    pub fn set_balance(&mut self, amnt: usize) {
        self.balance = amnt;
        // TODO(SEP): propgate?
        self.save().expect("Could not commit change for balance");
    }

    pub fn save(&mut self) -> std::io::Result<()> {
        let payload = toml::to_string(&self).map_err(std::io::Error::other)?;

        if let Some(f) = &mut self.file {
            f.set_len(0)?;
            f.seek(std::io::SeekFrom::Start(0))?;
            f.write_all(&payload.into_bytes())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "config path was None",
            ))
        }
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
        Self {
            balance: 0,
            file: None,
        }
    }
}

pub fn get_configuration_dir() -> Option<std::path::PathBuf> {
    directories_next::ProjectDirs::from("com", "spineda2019", "uba")
        .map(|pd| pd.config_dir().to_owned())
}

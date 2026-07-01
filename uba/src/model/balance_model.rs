#[derive(serde::Deserialize, serde::Serialize)]
pub struct TransactionModel {
    balance: i32,
    transaction: Vec<()>,
}

impl std::default::Default for TransactionModel {
    #[allow(clippy::derivable_impls)]
    fn default() -> Self {
        Self {
            balance: 0,
            transaction: Vec::new(),
        }
    }
}

impl std::fmt::Display for TransactionModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ balance: {} }}", self.balance)
    }
}

impl TransactionModel {
    pub fn from_file<T: std::io::Read>(f: &mut T) -> std::io::Result<Self> {
        let mut buf: String = String::with_capacity(512);
        f.read_to_string(&mut buf)?;
        toml::from_str(&buf).map_err(std::io::Error::other)
    }

    pub fn increment_and_get_balance(&mut self) -> i32 {
        self.balance += 1;
        self.get_balance()
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn serialize(&self) -> std::io::Result<String> {
        toml::to_string(self).map_err(std::io::Error::other)
    }
}

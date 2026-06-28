use uba_core::persistence::Config;

pub struct BalanceModel {
    balance: usize,
}

impl BalanceModel {
    pub fn new() -> Self {
        Self { balance: 0 }
    }

    pub(super) fn load_config(&mut self, config: &Config) {
        self.balance = config.get_balance();
    }

    pub fn increment_and_get_balance(&mut self) -> usize {
        self.balance += 1;
        self.get_balance()
    }

    pub fn get_balance(&self) -> usize {
        self.balance
    }
}

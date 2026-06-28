pub struct BalanceModel {
    balance: usize,
}

impl BalanceModel {
    pub fn new() -> Self {
        Self { balance: 0 }
    }

    pub fn increment_and_get_balance(&mut self) -> usize {
        self.balance += 1;
        self.balance
    }
}

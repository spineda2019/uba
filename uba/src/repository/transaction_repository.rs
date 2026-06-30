pub(crate) struct TransactionRepository<T: std::io::Write + uba_core::persistence::Truncate> {
    handle: T,
}

impl<T: std::io::Write + uba_core::persistence::Truncate> TransactionRepository<T> {
    pub fn new(handle: T) -> Self {
        Self { handle }
    }

    pub fn save(&mut self, serialized: &str) -> std::io::Result<()> {
        self.handle.seek(std::io::SeekFrom::Start(0))?;
        self.handle.set_len(0)?;
        self.handle.write_all(serialized.as_bytes())
    }

    pub fn borrow_handle(&mut self) -> &mut T {
        &mut self.handle
    }
}

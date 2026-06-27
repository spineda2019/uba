pub struct Logger<T: std::io::Write> {
    handle: T,
}

impl<T: std::io::Write> Logger<T> {
    pub fn new(handle: T) -> Self {
        Logger { handle }
    }

    pub fn log_error<E: std::error::Error>(&mut self, err: E) -> std::io::Result<()> {
        writeln!(self.handle, "ERR: {}", err)
    }

    pub fn log_msg(&mut self, msg: &str) -> std::io::Result<()> {
        writeln!(self.handle, "MSG: {}", msg)
    }
}

impl Logger<std::io::BufWriter<std::fs::File>> {
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Self> {
        let f: std::fs::File = std::fs::File::create(path)?;
        let writer: std::io::BufWriter<std::fs::File> = std::io::BufWriter::new(f);
        Ok(Logger { handle: writer })
    }
}

pub struct TransparentWriter {}

impl std::io::Write for TransparentWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Logger<TransparentWriter> {
    pub fn new_transparent() -> Self {
        Logger {
            handle: TransparentWriter {},
        }
    }
}

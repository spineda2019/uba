pub struct Logger<T: std::io::Write> {
    handle: T,
}

impl<T: std::io::Write> Logger<T> {
    pub fn new(handle: T) -> Self {
        Logger { handle }
    }

    pub fn log_error<E: std::error::Error>(
        &mut self,
        err: E,
        extra: Option<impl std::fmt::Display>,
    ) -> std::io::Result<()> {
        writeln!(self.handle, "ERR: {}", err)?;
        if let Some(diagnostic) = extra {
            writeln!(self.handle, "\t{}", diagnostic)
        } else {
            Ok(())
        }
    }

    pub fn log_msg(&mut self, msg: impl std::fmt::Display) -> std::io::Result<()> {
        writeln!(self.handle, "MSG: {}", msg)
    }

    pub fn log_warning(&mut self, msg: impl std::fmt::Display) -> std::io::Result<()> {
        writeln!(self.handle, "WARNING: {}", msg)
    }
}

impl Logger<std::io::BufWriter<std::fs::File>> {
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Self> {
        let f: std::fs::File = std::fs::File::create(path)?;
        let writer: std::io::BufWriter<std::fs::File> = std::io::BufWriter::new(f);
        Ok(Logger { handle: writer })
    }
}

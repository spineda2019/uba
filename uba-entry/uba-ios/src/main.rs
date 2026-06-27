use uba::launch;
use uba_core::log::Logger;

fn main() -> std::io::Result<()> {
    #[cfg(not(target_os = "ios"))]
    {
        compile_error!("uba-ios must be build targeting iOS");
    }

    let logger;
    #[cfg(target_env = "sim")]
    {
        logger = Logger::new(std::io::stderr());
    }
    #[cfg(not(target_env = "sim"))]
    {
        pub struct TransparentWriter {}

        impl std::io::Write for TransparentWriter {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                Ok(buf.len())
            }

            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }

        logger = Logger::new(TransparentWriter {});
    }

    launch(logger)
}

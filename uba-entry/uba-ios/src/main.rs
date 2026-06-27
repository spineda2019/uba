use uba_core::log::Logger;
slint::include_modules!();

fn main() -> std::io::Result<()> {
    let mut logger;
    #[cfg(target_env = "sim")]
    {
        logger = Logger::new(std::io::stderr());
    }
    #[cfg(not(target_env = "sim"))]
    {
        logger = Logger::new_transparent();
    }

    Ok(())
}

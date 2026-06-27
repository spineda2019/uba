#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use uba::launch;
use uba_core::log::Logger;

fn main() -> std::io::Result<()> {
    let logger = match Logger::from_file("uba.log") {
        Ok(l) => l,
        Err(e) => panic!("Could not create logger: {}", e),
    };

    launch(logger)
}

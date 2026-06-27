use uba_core::log::Logger;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    let logger = Logger::new(std::io::stdout());
    uba::launch(logger).unwrap();
}

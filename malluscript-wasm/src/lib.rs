use malluscript::run_source;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_malluscript(source: &str) -> String {
    match run_source(source) {
        Ok(output) => output,
        Err(err) => err,
    }
}

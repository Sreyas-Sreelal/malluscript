use wasm_bindgen::prelude::*;
use malluscript::run_source;

#[wasm_bindgen]
pub fn run_malluscript(source: &str) -> String {
    match run_source(source) {
        Ok(output) => output,
        Err(err) => err,
    }
}

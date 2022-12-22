use wasm_bindgen::prelude::*;

use malluscript::store_result;

#[wasm_bindgen]
pub fn run_malluscript(source:&str) -> String{
    store_result(&source).join("\n")
}
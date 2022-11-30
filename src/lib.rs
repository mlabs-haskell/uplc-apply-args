mod utils;

use wasm_bindgen::prelude::*;
// use uplc::apply_params_to_script as uplc_apply_params_to_script;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn strange(x : u32) -> u32 {
    return x + 1
}

#[wasm_bindgen]
pub fn apply_params_to_script(
    params_bytes: &[u8], // PlutusData array
    plutus_script_bytes: &[u8],
) -> Result<Vec<u8>, String> {
    match uplc::tx::apply_params_to_script(params_bytes, plutus_script_bytes) {
        Ok(a) => Ok(a),
        Err(e) => Err("error".to_string())
    }
}
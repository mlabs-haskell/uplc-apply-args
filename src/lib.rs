mod utils;

use uplc::{tx::error::Error, PlutusData};
use wasm_bindgen::prelude::*;
// use uplc::apply_params_to_script as uplc_apply_params_to_script;
use uplc::{
    ast::{DeBruijn, Program},
    // machine::cost_model::ExBudget,
    // PlutusData,
};
use pallas_primitives::{Fragment};

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
pub fn apply_params_array_to_script(
    params_bytes: &[u8], // PlutusData array
    plutus_script_bytes: &[u8],
) -> Result<Vec<u8>, String> {
    match uplc::tx::apply_params_to_script(params_bytes, plutus_script_bytes) {
        Ok(a) => Ok(a),
        Err(_e) => Err("error".to_string())
    }
}


pub fn apply_params_to_script(
    params_bytes: Vec<&[u8]>, // PlutusData array
    plutus_script_bytes: &[u8],
) -> Result<Vec<u8>, Error> {

    let mut buffer = Vec::new();
    let mut program = Program::<DeBruijn>::from_cbor(plutus_script_bytes, &mut buffer)?;

    for param in params_bytes {
        let arg = PlutusData::decode_fragment(param).unwrap();

        program = program.apply_data(arg);
    }

    match program.to_cbor() {
        Ok(res) => Ok(res),
        Err(_e) => Err(Error::ApplyParamsError),
    }
}

use get_config::{get_config, get_config_deserialize_input};
use serde::Serialize;
use wasm_bindgen::prelude::*;

pub use get_config::{GetConfigError, GetConfigParams};

/// Requires the input to be manually JSON-serialized to match the `get_config::GetConfigParams` struct.
#[wasm_bindgen(js_name = getConfigDeserializeInput)]
pub fn get_config_deserialize_input_wasm(params: String) -> Result<String, JsError> {
    get_config_deserialize_input(&params).map_err(|err| to_js_error(&err))
}

/// Requires the input as an object that matches the `get_config::GetConfigParams` struct.
#[wasm_bindgen(js_name = getConfigTyped)]
pub fn get_config_typed_wasm(params: get_config::GetConfigParams) -> Result<String, JsError> {
    get_config(params).map_err(|err| to_js_error(&err))
}

fn to_js_error<T>(err: &T) -> JsError
where
    T: Serialize + ?Sized,
{
    // Serialization can panic if T's implementation of Serialize decides to fail,
    // or if T contains a map with non-string keys. (That's not our case here.)
    JsError::new(&serde_json::to_string(&err).unwrap())
}

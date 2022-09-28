use core::panic;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use tsify::Tsify;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct GetConfigParams {
    pub prisma_schema: String,

    #[serde(default)]
    pub trigger_error: bool,

    #[serde(default)]
    env: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct GetConfigError {
    pub error_code: Option<String>,
    pub message: String,
}

pub fn get_config_deserialize_input(params: &str) -> Result<String, String> {
    let params: GetConfigParams = match serde_json::from_str(params) {
        Ok(params) => params,
        Err(err) => {
            panic!("Error parsing input JSON: {}", err);
        }
    };

    get_config(params).map_err(|err| serde_json::to_string(&err).unwrap())
}

pub fn get_config(params: GetConfigParams) -> Result<String, GetConfigError> {
    if params.trigger_error {
        Err(GetConfigError {
            error_code: Some("P1012".to_owned()),
            message: "This is an error".to_owned(),
        })
    } else {
        Ok("ok".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Error parsing input JSON: missing field `prismaSchema`")]
    fn test_deserialize_input_err() {
        let params = r#"{"prisma_schema": "foo"}"#;
        get_config_deserialize_input(params);
    }

    #[test]
    fn result_ok() {
        let params = GetConfigParams {
            prisma_schema: "schema.prisma".to_owned(),
            trigger_error: false,
            env: HashMap::new(),
        };
        let result = get_config(params);
        assert_eq!(result, Ok("ok".to_owned()));
    }

    #[test]
    fn result_err() {
        let result = get_config(GetConfigParams {
            prisma_schema: "schema.prisma".to_owned(),
            trigger_error: true,
            env: HashMap::new(),
        });
        assert_eq!(
            result,
            Err(GetConfigError {
                error_code: Some("P1012".to_owned()),
                message: "This is an error".to_owned(),
            })
        );
    }
}

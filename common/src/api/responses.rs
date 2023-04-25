use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use super::errors::AwsError;

#[derive(Serialize, Deserialize)]
pub struct DeployedModulesResponse {
    pub modules: Vec<GetModulesResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct GetModulesResponse {
    pub id: i32,
    pub module_hash: String,
    pub functions: Vec<DeployedFunctionResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct DeployedFunctionResponse {
    pub function: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetCreditsResponse {
    pub credits: i32,
}

#[derive(Serialize, Deserialize)]
pub struct DeployModuleResponse {
    pub mod_hash: String,
}

pub struct CallFunctionResponse {
    pub return_value: Vec<wasmer::Value>,
}

impl IntoResponse for CallFunctionResponse {
    fn into_response(self) -> axum::response::Response {
        let values = self
            .return_value
            .iter()
            .map(|v| match v {
                wasmer::Value::I32(x) => Ok(serde_json::Value::from(*x)),
                wasmer::Value::F32(x) => Ok(serde_json::Value::from(*x)),
                _ => Err(AwsError::UnimplementedWasmType),
            })
            .collect::<Result<Vec<_>, _>>();

        match values {
            Ok(v) => axum::Json::from(serde_json::json!({ "return_value": v })).into_response(),
            Err(e) => e.into_response(),
        }
    }
}

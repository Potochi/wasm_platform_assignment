use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub enum AwsError {
    UnknownServerError,
    InvalidCredentials,
    DuplicateUsername,
    Unauthorized,
    NotFound(Box<axum::http::Uri>),
    DuplicateFunction,
    InvalidWasmBase64,
    UnimplementedWasmType,
    EndpointNotFound(i32),
    FunctionNotFound(String),
    WasmTypeConversionError,
    WasmWrongParameterType((wasmer::Type, wasmer::Type)),
    WasmInstanceError(Box<wasmer::InstantiationError>),
    InvalidSignature(String),
    InvalidWasmModule,
    InsufficientCredits,
    PasswordTooShort,
    PasswordTooWeak,
    JwtSignatureFailure,
}

impl IntoResponse for AwsError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AwsError::InvalidCredentials => (
                StatusCode::BAD_REQUEST,
                axum::Json::from(serde_json::json!({"error": "invalid credentials"})),
            ),
            AwsError::DuplicateUsername => (
                StatusCode::BAD_REQUEST,
                axum::Json::from(serde_json::json!({"error": "duplicate username"})),
            ),
            AwsError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                axum::Json::from(serde_json::json!({"error": "unauthorized"})),
            ),
            AwsError::UnknownServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json::from(serde_json::json!({"error": "server error"})),
            ),
            AwsError::NotFound(uri) => (
                StatusCode::NOT_FOUND,
                axum::Json::from(serde_json::json!({ "error": format!("{uri} not found") })),
            ),
            AwsError::DuplicateFunction => (
                StatusCode::BAD_REQUEST,
                axum::Json::from(serde_json::json!({
                    "error": format!("duplicate deployment")
                })),
            ),
            AwsError::InvalidWasmBase64 => (
                StatusCode::BAD_REQUEST,
                axum::Json::from(serde_json::json!({
                    "error": format!("invalid wasm code base64")
                })),
            ),
            AwsError::UnimplementedWasmType => (
                StatusCode::BAD_REQUEST,
                axum::Json::from(serde_json::json!({
                    "error": format!("unimplemented wasm type")
                })),
            ),
            AwsError::EndpointNotFound(id) => (
                StatusCode::NOT_FOUND,
                axum::Json::from(serde_json::json!({
                    "error": format!("endpoint {end} not found", end = id)
                })),
            ),
            AwsError::FunctionNotFound(func) => (
                StatusCode::NOT_FOUND,
                axum::Json::from(serde_json::json!({
                    "error": format!("function {func} not found")
                })),
            ),
            AwsError::WasmTypeConversionError => (
                StatusCode::BAD_REQUEST,
                axum::Json::from(serde_json::json!({
                    "error": format!("type conversion failed on parameters")
                })),
            ),
            AwsError::WasmInstanceError(e) => (
                StatusCode::BAD_REQUEST,
                axum::Json::from(serde_json::json!({ "error": format!("{}", e) })),
            ),

            AwsError::InvalidSignature(sig) => (
                StatusCode::BAD_REQUEST,
                axum::Json::from(serde_json::json!({
                    "error": format!("signature {sig} is invalid")
                })),
            ),
            AwsError::InvalidWasmModule => (
                StatusCode::BAD_REQUEST,
                axum::Json::from(serde_json::json!({
                    "error": format!("invalid wasm module")
                })),
            ),
            AwsError::InsufficientCredits => (
                StatusCode::PAYMENT_REQUIRED,
                axum::Json::from(serde_json::json!({
                    "error": format!("insufficient credits")
                })),
            ),
            AwsError::PasswordTooShort => (
                StatusCode::BAD_REQUEST,
                axum::Json::from(serde_json::json!({
                    "error": format!("password too short")
                })),
            ),
            AwsError::PasswordTooWeak => (
                StatusCode::BAD_REQUEST,
                axum::Json::from(serde_json::json!({ "error": format!("password too weak") })),
            ),
            AwsError::WasmWrongParameterType((e, p)) => (
                StatusCode::BAD_REQUEST,
                axum::Json::from(serde_json::json!({
                    "error": format!("expected type {e} but got type {p}")
                })),
            ),
            AwsError::JwtSignatureFailure => (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json::from(serde_json::json!({
                    "error": format!("failed to sign token")
                })),
            ),
        }
        .into_response()
    }
}

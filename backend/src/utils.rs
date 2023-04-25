use lazy_static::lazy_static;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use wasmer::{wasmparser::Operator, ModuleMiddleware};

lazy_static! {
    pub static ref WASM_COST_FUNCTION: Arc<dyn ModuleMiddleware> =
        Arc::new(wasmer_middlewares::Metering::new(10, wasm_cost_function));
}

// TODO(Livian): More sophisticated cost function
pub fn wasm_cost_function(_: &Operator) -> u64 {
    1
}

pub fn password_secure_check(pass: &str) -> bool {
    pass.chars().any(|c| c.is_ascii_digit())
        && pass.chars().any(|c| c.is_uppercase())
        && pass.chars().any(|c| c.is_lowercase())
        && pass.chars().any(|c| !c.is_alphabetic())
}

#[derive(Clone)]
pub struct DbConn(pub Arc<DatabaseConnection>);

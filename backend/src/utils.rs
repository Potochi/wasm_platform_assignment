use lazy_static::lazy_static;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use wasmer::{wasmparser::Operator, ModuleMiddleware};

lazy_static! {
    pub static ref WASM_COST_FUNCTION: Arc<dyn ModuleMiddleware> =
        Arc::new(wasmer_middlewares::Metering::new(10, wasm_cost_function));
}

pub fn wasm_cost_function(op: &Operator) -> u64 {
    match op {
        Operator::LocalGet { local_index: _ } => 1,
        Operator::LocalSet { local_index: _ } => 2,
        Operator::LocalTee { local_index: _ } => 3,
        Operator::GlobalGet { global_index: _ } => 4,
        Operator::GlobalSet { global_index: _ } => 5,
        Operator::I32Load { memarg: _ } => 6,
        Operator::I64Load { memarg: _ } => 7,
        Operator::F32Load { memarg: _ } => 8,
        Operator::F64Load { memarg: _ } => 9,
        Operator::F32x4RelaxedDotBf16x8AddF32x4 => 100_000,
        _ => 1,
    }
}

pub fn password_secure_check(pass: &str) -> bool {
    pass.chars().any(|c| c.is_ascii_digit())
        && pass.chars().any(|c| c.is_uppercase())
        && pass.chars().any(|c| c.is_lowercase())
        && pass.chars().any(|c| !c.is_alphabetic())
}

#[derive(Clone)]
pub struct DbConn(pub Arc<DatabaseConnection>);

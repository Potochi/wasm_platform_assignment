use aws_common::api::{
    errors::AwsError, requests::CallFunctionBody, responses::CallFunctionResponse,
};
use axum::Extension;
use sea_orm::{ConnectionTrait, DbErr, TransactionError, TransactionTrait};
use sea_query::{Expr, Query};
use std::sync::Arc;
use wasmer::{imports, CompilerConfig, EngineBuilder, Instance, Module, Store};
use wasmer_middlewares::metering::{get_remaining_points, set_remaining_points, MeteringPoints};

use crate::{
    extractors::{ModuleFunctionExtract, WalletExtract},
    ffi::WasmFFIConverter,
    metrics::{FUNCTION_CALLS, FUNCTION_CALL_RESPONSE_TIME},
    migrator::m20230329_000003_wallets_table::Wallet,
    utils::{wasm_cost_function, DbConn},
};

pub async fn call_function(
    ModuleFunctionExtract { module, function }: ModuleFunctionExtract,
    WalletExtract(wallet): WalletExtract,
    Extension(DbConn(db)): Extension<DbConn>,
    axum::extract::Json(ctx): axum::extract::Json<CallFunctionBody>,
) -> Result<CallFunctionResponse, AwsError> {
    let params = function.to_wasm_params(&ctx.params)?;

    let _ = FUNCTION_CALL_RESPONSE_TIME.start_timer();

    let mut compiler_config = wasmer_compiler_cranelift::Cranelift::default();
    compiler_config.push_middleware(Arc::new(wasmer_middlewares::Metering::new(
        10,
        wasm_cost_function,
    )));

    let mut store = Store::new(EngineBuilder::new(compiler_config));

    let module = Module::new(&store, &module.wasm_code).map_err(|_| AwsError::InvalidWasmModule)?;

    let instance = Instance::new(&mut store, &module, &imports! {})
        .map_err(|e| AwsError::WasmInstanceError(Box::new(e)))?;

    set_remaining_points(&mut store, &instance, wallet.credits as u64);

    let func = instance
        .exports
        .get_function(&function.name)
        .map_err(|_| AwsError::FunctionNotFound(function.name.clone()))?;

    let params = params.iter().map(|x| x.0.clone()).collect::<Vec<_>>();

    let result = func.call(&mut store, &params).map_err(|e| {
        tracing::error!("Func call {e:#?}");

        match get_remaining_points(&mut store, &instance) {
            MeteringPoints::Remaining(_) => AwsError::UnknownServerError,
            MeteringPoints::Exhausted => AwsError::InsufficientCredits,
        }
    })?;

    // Exhausted branch should never be reached because
    // the remaining credits are checked when extracting the
    // result above and an error is returned if the credits
    // are exhausted
    let amt = match get_remaining_points(&mut store, &instance) {
        MeteringPoints::Remaining(x) => {
            Ok(i32::try_from(x).map_err(|_| AwsError::UnknownServerError)?)
        }
        MeteringPoints::Exhausted => unreachable!(),
    }?;

    let used = wallet.credits - amt;

    tracing::info!("used credits {used:#?}");

    db.transaction(|txn| {
        Box::pin(async move {
            let mut update_wallet_query = Query::update();

            update_wallet_query
                .table(Wallet::Table)
                .value(Wallet::Credits, Expr::col(Wallet::Credits).sub(used))
                .and_where(Expr::col(Wallet::Credits).gte(used))
                .and_where(Expr::col(Wallet::UserId).eq(wallet.user_id));

            let builder = txn.get_database_backend();
            let res = txn.execute(builder.build(&update_wallet_query)).await?;

            match res.rows_affected() {
                1 => Ok(()),
                _ => Err(DbErr::Custom("insufficent credits".to_string())),
            }
        })
    })
    .await
    .map_err(|e| {
        tracing::error!("transacetion {e:#?}");
        match e {
            TransactionError::Transaction(DbErr::Custom(_)) => AwsError::InsufficientCredits,
            _ => AwsError::UnknownServerError,
        }
    })?;

    FUNCTION_CALLS.inc();

    Ok(CallFunctionResponse {
        return_value: result[..function.get_ret_types()?.len()].to_vec(),
    })
}

use aws_common::api::{
    errors::AwsError,
    responses::{
        DeployModuleResponse, DeployedFunctionResponse, DeployedModulesResponse, GetModulesResponse,
    },
};
use axum::{
    body,
    extract::Path,
    http::{StatusCode, Uri},
    Extension,
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, QueryFilter, TransactionError,
    TransactionTrait,
};

use sha2::{Digest, Sha256};

use crate::{
    auth::jwt::AwsClaims, entities, extractors::ModuleHashPathParam, ffi, utils::DbConn,
    ModuleCache,
};

fn wasmer_types_to_string(types: &[wasmer::Type]) -> Result<String, AwsError> {
    types
        .iter()
        .map(|x| TryInto::<&str>::try_into(ffi::Type(*x)))
        .collect::<Result<Vec<&str>, AwsError>>()
        .map(|x| x.join(","))
}

pub async fn deploy_module(
    claims: AwsClaims,
    Extension(DbConn(db)): Extension<DbConn>,
    // axum::extract::Json(deploy_data): axum::extract::Json<DeployModuleBody>,
    data: body::Bytes,
) -> Result<(StatusCode, axum::Json<DeployModuleResponse>), AwsError> {
    let code = data.to_vec();

    let code_hash = format!("{:x}", Sha256::digest(&code));

    let inside_hash = code_hash.clone();

    let engine = wasmer::Store::default();
    let module =
        wasmer::Module::from_binary(&engine, &code).map_err(|_| AwsError::InvalidWasmModule)?;

    let mut exports = module
        .exports()
        .filter_map(|x| {
            let name = x.name().to_string();
            let fnc = x.ty().func()?.clone();

            Some((name, fnc))
        })
        .map(|(name, fnc)| {
            let p = wasmer_types_to_string(fnc.params())?;
            let r = wasmer_types_to_string(fnc.results())?;

            Ok(entities::function::ActiveModel {
                name: ActiveValue::set(name),
                signature: ActiveValue::set(format!("{p}->{r}")),
                ..Default::default()
            })
        })
        .collect::<Result<Vec<_>, AwsError>>()?;

    db.transaction::<_, _, DbErr>(|txn| {
        Box::pin(async move {
            let added_endpoint = entities::module::ActiveModel {
                owner_id: ActiveValue::set(claims.uid),
                wasm_code: ActiveValue::set(code),
                code_hash: ActiveValue::set(inside_hash),
                ..Default::default()
            }
            .save(txn)
            .await?;

            for e in exports.iter_mut() {
                e.module_id = added_endpoint.id.clone();
            }

            entities::function::Entity::insert_many(exports)
                .exec(txn)
                .await?;

            Ok(())
        })
    })
    .await
    .map_err(|_| AwsError::DuplicateFunction)?;

    Ok((
        StatusCode::CREATED,
        axum::Json::from(DeployModuleResponse {
            mod_hash: code_hash.to_string(),
        }),
    ))
}

pub async fn get_deployed_modules(
    claims: AwsClaims,
    Extension(DbConn(db)): Extension<DbConn>,
) -> Result<axum::Json<DeployedModulesResponse>, AwsError> {
    let modules = entities::module::Entity::find()
        .filter(entities::module::Column::OwnerId.eq(claims.uid))
        .all(&*db)
        .await
        .map_err(|_| AwsError::UnknownServerError)?;

    let mut deployments = DeployedModulesResponse {
        modules: Vec::new(),
    };

    for module in modules {
        let functions = entities::function::Entity::find()
            .filter(entities::function::Column::ModuleId.eq(module.id))
            .all(&*db)
            .await
            .map_err(|_| AwsError::UnknownServerError)?
            .iter()
            .map(|f| DeployedFunctionResponse {
                function: f.name.clone(),
                signature: f.signature.clone(),
            })
            .collect::<Vec<_>>();

        deployments.modules.push(GetModulesResponse {
            id: module.id,
            module_hash: module.code_hash.clone(),
            functions,
        })
    }

    Ok(axum::Json::from(deployments))
}

pub async fn delete_module(
    claims: AwsClaims,
    Extension(DbConn(db)): Extension<DbConn>,
    Extension(_cache): Extension<ModuleCache>,
    Path(ModuleHashPathParam { id }): Path<ModuleHashPathParam>,
    uri: Uri,
) -> Result<(), AwsError> {
    db.transaction::<_, _, DbErr>(|txn| {
        Box::pin(async move {
            let res = entities::module::Entity::delete(entities::module::ActiveModel {
                id: ActiveValue::set(id),
                owner_id: ActiveValue::set(claims.uid),
                ..Default::default()
            })
            .exec(txn)
            .await?;

            if res.rows_affected == 1 {
                Ok(())
            } else {
                Err(DbErr::Custom("not found".to_string()))
            }
        })
    })
    .await
    .map_err(|e| match e {
        TransactionError::Transaction(DbErr::Custom(_)) => AwsError::NotFound(Box::new(uri)),
        _ => AwsError::UnknownServerError,
    })?;

    Ok(())
}

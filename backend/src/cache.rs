use std::collections::HashMap;
use std::sync::Arc;

use aws_common::api::errors::AwsError;
use tokio::sync::RwLock;

use wasmer::{Module, Store};

#[derive(Clone, Default)]
pub struct ModuleCache(Arc<RwLock<HashMap<i32, Vec<u8>>>>);

impl ModuleCache {
    pub async fn get_or_insert(
        &self,
        store: &Store,
        id: i32,
        wasm_code: &Vec<u8>,
    ) -> Result<Vec<u8>, AwsError> {
        {
            let mod_cache = self.0.read().await;

            if let Some(code) = mod_cache.get(&id) {
                return Ok(code.clone());
            }
        }

        let mut mod_cache = self.0.write().await;
        let new_module = Module::new(store, wasm_code).map_err(|_| AwsError::InvalidWasmModule)?;

        let bytes = new_module
            .serialize()
            .map_err(|_| AwsError::UnknownServerError)?
            .to_vec();

        mod_cache.insert(id, bytes.clone());

        Ok(bytes)
    }

    pub async fn remove(&self, id: i32) {
        let mut mod_cache = self.0.write().await;

        mod_cache.remove(&id);
    }
}

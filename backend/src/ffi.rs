use aws_common::api::errors::AwsError;
use std::ops::{Deref, DerefMut};

use crate::entities;

#[derive(Debug)]
pub struct Type(pub wasmer::Type);

#[derive(Debug)]
pub struct Value(pub wasmer::Value);

impl Deref for Type {
    type Target = wasmer::Type;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Type {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Value {
    type Target = wasmer::Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Value {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait WasmFFIConverter {
    fn get_param_types(&self) -> Result<Vec<Type>, AwsError>;
    fn get_ret_types(&self) -> Result<Vec<Type>, AwsError>;
    fn to_wasm_params(&self, params: &[serde_json::Value]) -> Result<Vec<Value>, AwsError>;
}

impl WasmFFIConverter for entities::function::Model {
    fn get_param_types(&self) -> Result<Vec<Type>, AwsError> {
        let (params, _) = self
            .signature
            .split_once("->")
            .ok_or_else(|| AwsError::InvalidSignature(self.signature.to_string()))?;

        params
            .split(',')
            .filter(|p| !p.is_empty())
            .map(|x| x.try_into())
            .collect::<Result<Vec<_>, _>>()
    }

    fn get_ret_types(&self) -> Result<Vec<Type>, AwsError> {
        let (_, ret) = self
            .signature
            .split_once("->")
            .ok_or_else(|| AwsError::InvalidSignature(self.signature.to_string()))?;

        ret.split(',')
            .filter(|p| !p.is_empty())
            .map(|x| x.try_into())
            .collect::<Result<Vec<_>, _>>()
    }

    fn to_wasm_params(&self, params: &[serde_json::Value]) -> Result<Vec<Value>, AwsError> {
        let param_types = self.get_param_types()?;

        let params: Vec<Value> = params
            .iter()
            .zip(param_types.iter())
            .map(|(v, t)| match v {
                serde_json::Value::Number(inner) => {
                    let v = if inner.is_i64() {
                        wasmer::Value::from(
                            i32::try_from(
                                inner
                                    .as_i64()
                                    .ok_or_else(|| AwsError::UnimplementedWasmType)?,
                            )
                            .map_err(|_| AwsError::WasmTypeConversionError)?,
                        )
                    } else if inner.is_u64() {
                        wasmer::Value::from(
                            i32::try_from(
                                inner
                                    .as_u64()
                                    .ok_or_else(|| AwsError::UnimplementedWasmType)?,
                            )
                            .map_err(|_| AwsError::WasmTypeConversionError)?,
                        )
                    } else if inner.is_f64() {
                        wasmer::Value::from(
                            inner
                                .as_f64()
                                .ok_or_else(|| AwsError::UnimplementedWasmType)?
                                as f32,
                        )
                    } else {
                        return Err(AwsError::UnimplementedWasmType);
                    };

                    // Check if the type we got is the type the function expects
                    if v.ty() != t.0 {
                        return Err(AwsError::WasmWrongParameterType((t.0, v.ty())));
                    }

                    Ok(Value(v))
                }
                _ => Err(AwsError::UnimplementedWasmType),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(params)
    }
}

impl TryInto<&str> for Type {
    type Error = AwsError;

    fn try_into(self) -> Result<&'static str, Self::Error> {
        match self.0 {
            wasmer::Type::I32 => Ok("i32"),
            wasmer::Type::F32 => Ok("f32"),
            _ => Err(AwsError::UnimplementedWasmType),
        }
    }
}

impl TryFrom<&str> for Type {
    type Error = AwsError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "i32" => Ok(Type(wasmer::Type::I32)),
            "f32" => Ok(Type(wasmer::Type::F32)),
            _ => Err(AwsError::UnimplementedWasmType),
        }
    }
}

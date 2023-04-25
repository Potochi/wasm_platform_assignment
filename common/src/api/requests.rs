use serde::Deserialize;

#[derive(Deserialize)]
pub struct EpxortedFunction {
    pub name: String,
    pub signature: String,
}

#[derive(Deserialize)]
pub struct DeployModuleBody {
    pub code_base64: String,
}

#[derive(Deserialize)]
pub struct CallFunctionBody {
    pub params: Vec<serde_json::Value>,
}

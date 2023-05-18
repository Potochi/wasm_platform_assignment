use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AwsClaims {
    pub sub: String,
    pub exp: usize,
    pub uid: i32,
}

#[derive(Serialize, Deserialize)]
struct SignRequest {
    claims: AwsClaims,
}

#[derive(Serialize, Deserialize)]
struct SignResponse {}

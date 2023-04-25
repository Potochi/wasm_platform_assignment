use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Register {
    pub username: String,
    pub password: String,
}

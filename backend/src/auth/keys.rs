use jsonwebtoken::{DecodingKey, EncodingKey};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref JWT_ENCODING_KEY: EncodingKey = EncodingKey::from_base64_secret(
        &std::env::var("JWT_BASE64_SECRET")
            .unwrap_or("cGVuaXNiZW5pcw==".to_string())
            
    )
    .expect("secret to be valid");
    pub static ref JWT_DECODING_KEY: DecodingKey = DecodingKey::from_base64_secret(
        &std::env::var("JWT_BASE64_SECRET")
            .unwrap_or("cGVuaXNiZW5pcw==".to_string())
            
    )
    .expect("secret to be valid");
}

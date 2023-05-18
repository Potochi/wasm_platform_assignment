use jsonwebtoken::DecodingKey;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref JWT_DECODING_KEY: DecodingKey = DecodingKey::from_ec_pem(
        std::fs::read_to_string(
            std::env::var("JWT_PUBLIC_KEY_PATH").expect("env var to be present")
        )
        .expect("to be able to read public key file")
        .as_bytes()
    )
    .expect("Public key to be valid");
}

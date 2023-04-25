pub const DEFAULT_DB_URL: &str = "sqlite:./testdb.sqlite?mode=rwc";
pub const JWT_TOKEN_VALIDITY: std::time::Duration = std::time::Duration::from_secs(60 * 60 * 48);
pub const INITIAL_WALLET_CREDITS: i32 = 1_000_000;
pub const MINIMUM_PASSWORD_LENGTH: usize = 12;

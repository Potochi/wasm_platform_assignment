use lazy_static::lazy_static;
use prometheus::{register_counter, register_gauge, register_histogram, Counter, Gauge, Histogram};

lazy_static! {
    pub static ref FUNCTION_CALLS: Counter =
        register_counter!("function_call_counter", "Functions called by all users")
            .expect("to create counter");
    pub static ref ACTIVE_USERS: Gauge =
        register_gauge!("active_users", "Number of registered users").expect("to create gauge");
    pub static ref FUNCTION_CALL_RESPONSE_TIME: Histogram =
        register_histogram!("function_call_response_time", "Duration of function calls")
            .expect("to create histogram");
    pub static ref WASM_CODE_SIZE: Gauge =
        register_gauge!("wasm_code_size", "Size of the stored WASM code").expect("to create gauge");
}

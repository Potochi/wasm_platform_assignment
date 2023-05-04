use axum::http::StatusCode;

pub async fn get_metrics() -> Result<String, StatusCode> {
    let mut data = String::new();
    let encoder = prometheus::TextEncoder::new();

    encoder
        .encode_utf8(&prometheus::gather(), &mut data)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(data)
}

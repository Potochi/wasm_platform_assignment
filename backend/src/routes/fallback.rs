use aws_common::api::errors::AwsError;

pub async fn fallback(uri: axum::http::Uri) -> AwsError {
    AwsError::NotFound(Box::new(uri))
}

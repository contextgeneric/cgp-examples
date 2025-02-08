use axum::http::StatusCode;

#[derive(Debug)]
pub struct AppError {
    pub status_code: StatusCode,
    pub detail: anyhow::Error,
}

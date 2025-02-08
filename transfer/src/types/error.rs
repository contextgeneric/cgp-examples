use axum::http::StatusCode;

pub struct AppError {
    pub status_code: StatusCode,
    pub detail: anyhow::Error,
}

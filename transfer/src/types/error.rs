use axum::http::StatusCode;

/// The concrete error this deployment uses, pairing an HTTP status with a detail. `MockApp`
/// wires it as the app's abstract `Error` type, and the HTTP layer turns it into a response.
#[derive(Debug)]
pub struct AppError {
    pub status_code: StatusCode,
    pub detail: anyhow::Error,
}

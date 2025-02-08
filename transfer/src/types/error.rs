pub struct ApiError {
    pub status_code: u16,
    pub detail: anyhow::Error,
}

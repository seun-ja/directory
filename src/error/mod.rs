#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("an error occurred with the database")]
    Sqlx(#[from] sqlx::Error),
}

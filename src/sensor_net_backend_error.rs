#[derive(thiserror::Error, Debug)]
pub enum SensorNetBackendError {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

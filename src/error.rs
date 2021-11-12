use thiserror::Error;

#[derive(Error, Debug)]
pub enum FunctionalError {
    #[error("failed to initialize: error code {0}")]
    FailedInitialization(i32),
    #[error("invalid functional ID")]
    InvalidID,
    #[error("invalid functional name")]
    InvalidName,
}

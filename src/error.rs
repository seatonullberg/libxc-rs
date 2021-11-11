use thiserror::Error;

#[derive(Error, Debug)]
pub enum FunctionalError {
    #[error("invalid functional ID")]
    InvalidID,
    #[error("invalid functional name")]
    InvalidName,
}

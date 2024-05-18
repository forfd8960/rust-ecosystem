use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidateError {
    #[error("exceed limit {0}, {1}")]
    ExceedLimit(u32, u32),
    #[error("file not found: {0}")]
    IOError(#[from] std::io::Error),
    #[error("parse num error: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
}

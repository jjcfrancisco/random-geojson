use thiserror::Error;

pub type RandomGeojsonResult<T> = Result<T, RandomGeojsonError>;

#[derive(Error, Debug)]
pub enum RandomGeojsonError {
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
}

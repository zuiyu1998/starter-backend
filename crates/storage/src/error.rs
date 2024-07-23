use abi::thiserror::{self, Error};
use serde_json::Error as SerdeError;

use std::io::Error as IoError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] IoError),
    #[error("io error: {0}")]
    SerdeError(#[from] SerdeError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

use figment::error::Error as FigmentError;
use thiserror::Error;

use std::io::Error as IoError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] IoError),

    #[error("figment error: {0}")]
    FigmentError(#[from] FigmentError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

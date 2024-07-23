use abi::{
    sea_orm::DbErr,
    thiserror::{self, Error},
};

use std::io::Error as IoError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] IoError),
    #[error("db error: {0}")]
    DbErr(#[from] DbErr),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

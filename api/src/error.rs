use abi::{
    thiserror::{self, Error},
    Error as AbiError,
};
use db::Error as DbError;
use doc_index::Error as DocIndexError;
use std::io::Error as IoError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] IoError),
    #[error("db error: {0}")]
    DbError(#[from] DbError),
    #[error("abi error: {0}")]
    AbiError(#[from] AbiError),
    #[error("doc index error: {0}")]
    DocIndexError(#[from] DocIndexError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

use abi::{
    thiserror::{self, Error},
    Error as AbiError,
};
use std::io::Error as IoError;
use tantivy::{error::TantivyError, query::QueryParserError};

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] IoError),
    #[error("abi error: {0}")]
    AbiError(#[from] AbiError),
    #[error("tantivy error: {0}")]
    TantivyError(#[from] TantivyError),
    #[error("tantivy query parser error: {0}")]
    QueryParserError(#[from] QueryParserError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

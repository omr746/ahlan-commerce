use thiserror::Error;
use crate::id::ProductId;

#[derive(Debug, Error, PartialEq)]
pub enum CatalogError {
    #[error("product handle '{0}' already exists")]
    DuplicateHandle(String),

    #[error("product {0} was not found")]
    NotFound(ProductId),
}
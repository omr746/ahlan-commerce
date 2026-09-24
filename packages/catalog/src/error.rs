use crate::id::ProductId;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CatalogError {
    #[error("product handle '{0}' already exists")]
    DuplicateHandle(String),

    #[error("product {0} was not found")]
    NotFound(ProductId),
    #[error("product storage failed")]
    Storage(#[source] sqlx::Error),
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CatalogDbError {
    #[error("database pool error: {0}")]
    Pool(#[from] deadpool_postgres::BuildError),

    #[error("database error: {0}")]
    Database(#[from] tokio_postgres::Error),

    #[error("product not found")]
    NotFound,
    #[error("database pool error: {0}")]
    PoolGet(#[from] deadpool_postgres::PoolError),

    #[error("duplicate product handle")]
    DuplicateHandle,
    #[error("invalid database configuration: {0}")]
    InvalidDatabaseUrl(String),
}

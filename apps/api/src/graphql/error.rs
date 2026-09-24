use async_graphql::{Error, ErrorExtensions};

use catalog_db::CatalogDbError;

pub fn validation_error(message: String) -> Error {
    Error::new(message).extend_with(|_, e| {
        e.set("code", "validation_error");
    })
}

pub fn graphql_error(error: CatalogDbError) -> Error {
    match error {
        CatalogDbError::NotFound => Error::new("product was not found").extend_with(|_, e| {
            e.set("code", "not_found");
        }),

        CatalogDbError::DuplicateHandle => Error::new("product handle is already in use")
            .extend_with(|_, e| {
                e.set("code", "duplicate_product_handle");
            }),

        CatalogDbError::Pool(_) | CatalogDbError::Database(_) | CatalogDbError::PoolGet(_) => {
            Error::new("a required dependency is unavailable").extend_with(|_, e| {
                e.set("code", "dependency_unavailable");
            })
        }

        CatalogDbError::InvalidDatabaseUrl(_) => Error::new("the server failed unexpectedly")
            .extend_with(|_, e| {
                e.set("code", "internal_error");
            }),
    }
}

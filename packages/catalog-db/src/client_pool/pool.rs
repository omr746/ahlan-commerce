use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

use crate::error::CatalogDbError;

pub async fn create_pool(database_url: &str) -> Result<Pool, CatalogDbError> {
    let config: tokio_postgres::Config = database_url.parse().map_err(|err| {
        CatalogDbError::InvalidDatabaseUrl(format!("invalid database URL: {err}"))
    })?;

    let manager = Manager::from_config(
        config,
        NoTls,
        ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        },
    );

    let pool = Pool::builder(manager)
        .max_size(10)
        .build()
        .map_err(CatalogDbError::Pool)?;

    // Eagerly check we can actually reach the database at startup,
    // rather than failing lazily on the first request.
    let _client = pool.get().await.map_err(CatalogDbError::PoolGet)?;

    Ok(pool)
}

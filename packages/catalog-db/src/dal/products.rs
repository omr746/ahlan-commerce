
use catalog::{Product, ProductCreate, ProductUpdate};
use chrono::{DateTime, FixedOffset, Utc};
use deadpool_postgres::Pool;
use uuid::Uuid;
use catalog::{Clock, IdGenerator, SystemClock, UuidV7Generator};
use crate::error::CatalogDbError;

use catalog_db_queries::queries::products::{
    create_product,
    list_products,
    list_published_products,
    update_product_publication,
    get_published_product_by_handle
};

#[derive(Clone)]
pub struct PgCatalog {
    pool: Pool,
}

impl PgCatalog {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    // ============================================================
    // CREATE PRODUCT
    // ============================================================

    pub async fn create_product(
        &self,
        input: ProductCreate,
        ids:  &dyn IdGenerator,
        clock: &dyn Clock,
    ) -> Result<Product, CatalogDbError> {
        let client = self.pool.get().await?;
               let id=ids.new_id();
      let now=clock.now();
        // Domain/application layer uses UTC.
        // Cornucopia generated code uses FixedOffset.
        let now_fixed = now.with_timezone(
            &FixedOffset::east_opt(0).unwrap(),
        );

        // Nullable publication timestamp.
        //
        // If published = true:
        //     published_at = Some(now)
        //
        // If published = false:
        //     published_at = NULL
        let published_at = if input.published {
            Some(now_fixed)
        } else {
            None
        };

        // PostgreSQL uses i32 for these columns,
        // while the domain uses u32.
        let price_cents = input.price_cents as i32;
        let inventory_quantity = input.inventory_quantity as i32;

        // Cornucopia nullable StringSql parameter:
        //
        // ProductCreate:
        //     Option<String>
        //
        // Cornucopia bind:
        //     &Option<T>
        //
        // as_deref() converts:
        //     Option<String>
        //
        // into:
        //     Option<&str>
        let description = input.description.as_deref();

        let result = create_product::create_product()
            .bind(
                &client,
                &id,
                &input.title,
                &input.handle,
                &description,
                &price_cents,
                &inventory_quantity,
                &input.published,
                &published_at,
                &now_fixed,
                &now_fixed,
            )
            .one()
            .await;
          let row = match result {
        Ok(row) => row,

        Err(err) => {
            if err.code() == Some(&tokio_postgres::error::SqlState::UNIQUE_VIOLATION) {
                return Err(CatalogDbError::DuplicateHandle);
            }

            return Err(CatalogDbError::Database(err));
        }
    };

        Ok(Product {
            id: row.id,
            title: row.title,
            handle: row.handle,
            description: row.description,
            price_cents: row.price_cents as u32,
            inventory_quantity: row.inventory_quantity as u32,
            published: row.published,

            // FixedOffset -> UTC
            published_at: row
                .published_at
                .map(|dt| dt.with_timezone(&Utc)),

            created_at: row.created_at.with_timezone(&Utc),
            updated_at: row.updated_at.with_timezone(&Utc),
        })
    }

    // ============================================================
    // LIST ALL PRODUCTS
    // ============================================================

    pub async fn list_products(
        &self,
    ) -> Result<Vec<Product>, CatalogDbError> {
        let client = self.pool.get().await?;

        let rows = list_products::list_products()
            .bind(&client)
            .all()
            .await
            .map_err(CatalogDbError::from)?;

        rows.into_iter()
            .map(product_from_list)
            .collect()
    }

    // ============================================================
    // LIST PUBLISHED PRODUCTS
    // ============================================================

    pub async fn list_published_products(
        &self,
    ) -> Result<Vec<Product>, CatalogDbError> {
        let client = self.pool.get().await?;

        let rows = list_published_products::list_published_products()
            .bind(&client)
            .all()
            .await
            .map_err(CatalogDbError::from)?;

        rows.into_iter()
            .map(product_from_published_list)
            .collect()
    }

    // ============================================================
    // UPDATE PRODUCT PUBLICATION
    // ============================================================

    pub async fn update_product_publication(
        &self,
        id: Uuid,
        input: ProductUpdate,
        clock:&dyn Clock,
    ) -> Result<Product, CatalogDbError> {
        let client = self.pool.get().await?;
          let now=clock.now();
        // UTC -> FixedOffset
        let now_fixed = now.with_timezone(
            &FixedOffset::east_opt(0).unwrap(),
        );

        // Keep NULL when the product is unpublished.
        let published_at = if input.published {
            Some(now_fixed)
        } else {
            None
        };

        let row = update_product_publication::update_product_publication()
            .bind(
                  &client,
        &input.published,
        &published_at,
        &now_fixed,
        &id,
            )
            .one()
            .await
            .map_err(CatalogDbError::from)?;

        Ok(Product {
            id: row.id,
            title: row.title,
            handle: row.handle,
            description: row.description,
            price_cents: row.price_cents as u32,
            inventory_quantity: row.inventory_quantity as u32,
            published: row.published,

            // FixedOffset -> UTC
            published_at: row
                .published_at
                .map(|dt| dt.with_timezone(&Utc)),

            created_at: row.created_at.with_timezone(&Utc),
            updated_at: row.updated_at.with_timezone(&Utc),
        })
    }
    pub async fn get_published_product_by_handle(
    &self,
    handle: &str,
) -> Result<Option<Product>, CatalogDbError> {
    let client = self.pool.get().await?;
    let row = get_published_product_by_handle::get_published_product_by_handle()
        .bind(&client, &handle)
        .opt()
        .await
        .map_err(CatalogDbError::from)?;

    Ok(row.map(|r| Product {
        id: r.id,
        title: r.title,
        handle: r.handle,
        description: r.description,
        price_cents: r.price_cents as u32,
        inventory_quantity: r.inventory_quantity as u32,
        published: r.published,
        published_at: r.published_at.map(|dt| dt.with_timezone(&Utc)),
        created_at: r.created_at.with_timezone(&Utc),
        updated_at: r.updated_at.with_timezone(&Utc),
    }))
}

}

// ================================================================
// MAPPERS
// ================================================================

fn product_from_list(
    row: list_products::ListProducts,
) -> Result<Product, CatalogDbError> {
    Ok(Product {
        id: row.id,
        title: row.title,
        handle: row.handle,
        description: row.description,
        price_cents: row.price_cents as u32,
        inventory_quantity: row.inventory_quantity as u32,
        published: row.published,

        // FixedOffset -> UTC
        published_at: row
            .published_at
            .map(|dt| dt.with_timezone(&Utc)),

        created_at: row.created_at.with_timezone(&Utc),
        updated_at: row.updated_at.with_timezone(&Utc),
    })
}

fn product_from_published_list(
    row: list_published_products::ListPublishedProducts,
) -> Result<Product, CatalogDbError> {
    Ok(Product {
        id: row.id,
        title: row.title,
        handle: row.handle,
        description: row.description,
        price_cents: row.price_cents as u32,
        inventory_quantity: row.inventory_quantity as u32,
        published: row.published,

        // FixedOffset -> UTC
        published_at: row
            .published_at
            .map(|dt| dt.with_timezone(&Utc)),

        created_at: row.created_at.with_timezone(&Utc),
        updated_at: row.updated_at.with_timezone(&Utc),
    })
}

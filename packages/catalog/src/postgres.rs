use chrono :: {DateTime,Utc};
use sqlx ::postgres::{PgPoolOptions,PgRow};

use sqlx ::{PgPool,Row};
use crate::clock::Clock;
use crate::error::CatalogError;
use crate::id::{IdGenerator, ProductId};
use crate::catalog::{Product, ProductCreate};

const SCHEMA: &str = include_str!("../../../db/schema/schema.sql");

#[derive(Clone)]
pub struct PgCatalog{
    pool:PgPool,
}
impl PgCatalog{
pub async fn connect(database_url:&str)->Result<Self,sqlx::Error>{
    let pool=PgPoolOptions::new()
    .max_connections(10)
    .connect(database_url)
    .await?;
  //  Self::ensure_schema(&pool).await?;
    Ok(Self{pool})
}
  pub fn from_pool(pool: PgPool) -> Self {
        Self { pool}
    }
   pub async fn ensure_schema(pool:&PgPool)->Result<(),sqlx::Error>{
    sqlx::query(SCHEMA).execute(pool).await?;
    Ok(())
   }

   pub async fn create_product(&self,input:ProductCreate,ids:&dyn IdGenerator,clock:&dyn Clock)->Result<Product,CatalogError>{

      let id=ids.new_id();
      let now=clock.now();
         let published_at = if input.published { Some(now) } else { None };
      let result=sqlx::query(
        "insert into products\
      (id,title,handle,price_cents,inventory_quantity,published,created_at,updated_at,description,published_at)\
      values($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)",
      
      ).bind(id)
        .bind(&input.title)
        .bind(&input.handle)
        .bind(input.price_cents as i32)
        .bind(input.inventory_quantity as i32)
        .bind(input.published)
        .bind(now)
        .bind(now)
        .bind(&input.description)
        .bind(published_at)
        .execute(&self.pool)
        .await;
        match result{
            Ok(_)=>Ok(Product {
                id,
                title: input.title,
                handle: input.handle,
                price_cents: input.price_cents,
                inventory_quantity: input.inventory_quantity,
                published: input.published,
                created_at: now,
                updated_at: now,
                description:input.description,
                published_at
            }),
            Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
                Err(CatalogError::DuplicateHandle(input.handle))
            }
            Err(err) => Err(CatalogError::Storage(err)),
        }


   }

   pub async fn list_products(&self)->Result<Vec<Product>,CatalogError>{
        
     let rows=sqlx::query(
        "select id, title, handle, description, price_cents, inventory_quantity, published, \
             published_at, created_at, updated_at \
             from products order by created_at asc",
     ).fetch_all(&self.pool).await
     .map_err(CatalogError::Storage)?;
    rows.into_iter().map(product_from_row).collect()
   }
  pub async fn get_product(&self, id: ProductId) -> Result<Product, CatalogError> {
        let row = sqlx::query(
            "select id, title, handle, description, price_cents, inventory_quantity, published, \
             published_at, created_at, updated_at \
             from products where id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(CatalogError::Storage)?;

        match row {
            Some(row) => product_from_row(row),
            None => Err(CatalogError::NotFound(id)),
        }
    }

}


fn product_from_row(row: PgRow) -> Result<Product, CatalogError> {
    Ok(Product {
        id: row.try_get("id").map_err(CatalogError::Storage)?,
        title: row.try_get("title").map_err(CatalogError::Storage)?,
        handle: row.try_get("handle").map_err(CatalogError::Storage)?,
        price_cents: row.try_get::<i32, _>("price_cents").map_err(CatalogError::Storage)? as u32,
        inventory_quantity: row
            .try_get::<i32, _>("inventory_quantity")
            .map_err(CatalogError::Storage)? as u32,
        published: row.try_get("published").map_err(CatalogError::Storage)?,
        created_at: row.try_get::<DateTime<Utc>, _>("created_at").map_err(CatalogError::Storage)?,
        updated_at: row.try_get::<DateTime<Utc>, _>("updated_at").map_err(CatalogError::Storage)?,
        published_at:row.try_get::<Option<DateTime<Utc>>,_>("published_at").map_err(CatalogError::Storage)?,
        description:row.try_get("description").map_err(CatalogError::Storage)?
    })
}
use crate::app::AppState;
use crate::graphql::product::Product;
use async_graphql::{Context, Object, Result};

pub struct Query;

#[Object]
impl Query {
    async fn products(&self, ctx: &Context<'_>) -> Result<Vec<Product>> {
        let state = ctx.data::<AppState>()?;

        let products = state
            .catalog
            .list_products()
            .await
            .map_err(crate::graphql::error::graphql_error)?;

        Ok(products.iter().map(Product::from).collect())
    }
}

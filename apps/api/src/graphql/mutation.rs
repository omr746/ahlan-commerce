use async_graphql::{Context, Object, Result};

use crate::app::AppState;
use crate::graphql::product::Product;
use crate::graphql::input::ProductCreateInput;
use crate::graphql::error::{graphql_error,validation_error};   

pub struct Mutation;

#[Object]
impl Mutation {
    async fn product_create(
        &self,
        ctx: &Context<'_>,
        input: ProductCreateInput,
    ) -> Result<Product> {
        let state=ctx.data::<AppState>()?;
        input.validate().map_err(validation_error)?;
        let input=input.into();
        let product=state.catalog
            .create_product(input,state.ids.as_ref(),state.clock.as_ref())
            .await
            .map_err(graphql_error)?;
        Ok(Product::from(&product))
      
    }
}
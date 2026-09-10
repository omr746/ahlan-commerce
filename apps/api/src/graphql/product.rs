use async_graphql::SimpleObject;
use crate::graphql::scalar::GraphQLDateTime;



#[derive(SimpleObject)]
#[graphql(rename_fields = "camelCase")]
pub struct Product {
    pub id: async_graphql::ID,
    pub title: String,
    pub handle: String,
    pub description: Option<String>,
    pub price_cents: i32,
    pub inventory_quantity: i32,
    pub published: bool,
    pub published_at: Option<GraphQLDateTime>,
    pub created_at: GraphQLDateTime,
    pub updated_at: GraphQLDateTime,
}

impl From<&catalog::Product> for Product {
    fn from(product: &catalog::Product) -> Self {
        Self {
            id: async_graphql::ID(product.id.to_string()),
            title: product.title.clone(),
            handle: product.handle.clone(),
            description: product.description.clone(),
            price_cents: product.price_cents as i32,
            inventory_quantity: product.inventory_quantity as i32,
            published: product.published,
            published_at: product.published_at.map(GraphQLDateTime),
            created_at: GraphQLDateTime(product.created_at),
            updated_at: GraphQLDateTime(product.updated_at),
        }
    }
}
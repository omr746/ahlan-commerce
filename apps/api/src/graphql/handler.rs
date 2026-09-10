use async_graphql_axum::{
    GraphQLRequest,
    GraphQLResponse,
};

use axum::extract::Extension;

use super::schema::AppSchema;


pub async fn graphql_handler(
    Extension(schema): Extension<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}   
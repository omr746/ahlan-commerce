use async_graphql::{
    EmptySubscription,
    Schema,
};

use crate::app::AppState;

use super::{
    mutation::Mutation,
    query::Query,
};

pub type AppSchema =
    Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema(state: AppState) -> AppSchema {
    Schema::build(
        Query,
        Mutation,
        EmptySubscription,
    )
    .data(state)
    .finish()
}

pub fn schema_sdl() -> String {
    Schema::build(Query, Mutation, EmptySubscription)
        .finish()
        .sdl()
}

pub mod error;
pub mod input;
pub mod mutation;
pub mod product;
pub mod query;
pub mod schema;
pub mod scalar;
pub mod handler;

pub use schema::{create_schema, AppSchema};
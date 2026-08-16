use std::sync::Arc;
use axum::{
    extract::{State,Json},
    routing::{get,post},
    Router,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use catalog::{Catalog, Clock, IdGenerator, SystemClock, UuidV7Generator};
use crate::handlers::{create_product, health, list_products};
use crate::config::Config;
use crate::routes;

#[derive(Clone)]
pub struct AppState{
    pub catalog:Arc<Mutex<Catalog>>,
    pub config:Arc<Config>,
    pub ids:Arc<dyn IdGenerator>,
    pub clock:Arc<dyn Clock>
}

impl AppState{
    pub fn new(config: Config)->Self{
        Self{
            catalog:Arc::new(Mutex::new(Catalog::new())),
            config:Arc::new(config),
            ids: Arc::new(UuidV7Generator),
            clock: Arc::new(SystemClock),
        }
    }
}





 pub fn create_router(state:AppState)->Router{
   Router::new()
   .route(routes::HEALTH,get(health))
     .route(
            routes::PRODUCTS,
            get(list_products).post(create_product),
        )
   .with_state(state)
}

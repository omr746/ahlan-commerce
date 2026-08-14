use std::sync::Arc;
use axum::{
    extract::{State,Json},
    routing::{get,post},
    Router,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::catalog::{Catalog,ProductCreate,ProductId};

#[derive(Clone)]
pub struct AppState{
    pub catalog:Arc<Mutex<Catalog>>
}

impl AppState{
    pub fn new()->Self{
        Self{
            catalog:Arc::new(Mutex::new(Catalog::new()))
        }
    }
}
#[derive(Serialize)]
struct HealthResponse{
    status:&'static str
}
async fn health()->Json<HealthResponse>{
    Json(HealthResponse{
    status:"ok"
    }
    )
}
#[derive(Serialize)]
struct ProductResponse{
    id:ProductId,
    title:String,
    handle:String,
    price_cents:u32,
    inventory_quantity:u32,
    published:bool
}
async fn create_product(
    State(state):State<AppState>,
    Json(input):Json<ProductCreate>
)-> (StatusCode,Json<ProductResponse>)
{
    let mut catalog=state.catalog.lock().await;
    let product=catalog.create_product(input);
    let response=ProductResponse{
        id:product.id,
        title:product.title.clone(),
        handle:product.handle.clone(),
        price_cents:product.price_cents,
        inventory_quantity:product.inventory_quantity,
        published:product.published
    };
    (StatusCode::CREATED,Json(response))
}

async fn list_products(
    State(state):State<AppState>
)->Json<Vec<ProductResponse>>{
let catalog=state.catalog.lock().await;
let products=catalog.list_products();
let response=products.iter().map(|product| ProductResponse{
    id:product.id,
    title:product.title.clone(),
    handle:product.handle.clone(),
    price_cents:product.price_cents,
    inventory_quantity:product.inventory_quantity,
    published:product.published
}).collect();
Json(response)
}





 pub fn create_router(state:AppState)->Router{
   Router::new()
   .route("/health",get(health))
     .route(
            "/api/products",
            get(list_products).post(create_product),
        )
   .with_state(state)
}

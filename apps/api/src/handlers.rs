
use axum::{
    extract::{State,Json},
    routing::{get,post},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use crate::app::AppState;
use catalog::{Catalog,ProductCreate,ProductId};

use crate::dto::{ProductCreateRequest, ProductResponse};

#[derive(Serialize)]
pub struct HealthResponse{
    status:&'static str
}
pub async  fn health()->Json<HealthResponse>{
    Json(HealthResponse{
    status:"ok"
    }
    )
}


pub async fn create_product(
    State(state):State<AppState>,
    Json(input):Json<ProductCreateRequest>
)-> (StatusCode,Json<ProductResponse>)
{
    let mut catalog=state.catalog.lock().await;
    let product=catalog.create_product(input.into(), state.ids.as_ref(), state.clock.as_ref());
    let response=product.into();
    (StatusCode::CREATED,Json(response))
}

pub async fn list_products(
    State(state):State<AppState>
)->Json<Vec<ProductResponse>>{
let catalog=state.catalog.lock().await;
let products=catalog.list_products();
let response=products.iter().map(|product|
    product.into()
).collect();
Json(response)
}
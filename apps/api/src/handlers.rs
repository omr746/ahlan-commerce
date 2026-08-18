
use axum::{
    extract::{State,Json,Path},
    routing::{get,post},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use crate::app::AppState;
use catalog::{Catalog,ProductCreate,ProductId};

use crate::error::AppError;
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
)-> Result<(StatusCode,Json<ProductResponse>),AppError>
{
    let request_id=state.ids.new_id();
     input
        .validate()
        .map_err(|message| AppError::validation(message, request_id))?;
    let mut catalog=state.catalog.lock().await;
    let product=catalog
    .create_product(input.into(), state.ids.as_ref(), state.clock.as_ref())
    .map_err(|err|AppError::from_catalog(err, request_id))?;
    let response=product.into();
    Ok((StatusCode::CREATED,Json(response)))
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


pub async fn get_product(
 State(state):State<AppState>,
 Path(id):Path<String>

)-> Result<Json<ProductResponse>,AppError>{
let request_id=state.ids.new_id();
  let product_id: ProductId = id
        .parse()
        .map_err(|_| AppError::validation(format!("'{id}' is not a valid product id."), request_id))?;

let catalog=state.catalog.lock().await;
let product=catalog
.get_product(product_id).map_err(|err|AppError::from_catalog(err, request_id))?;

Ok(Json(product.into()))

}


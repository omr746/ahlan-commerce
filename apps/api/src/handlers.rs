
use axum::{
     extract::{Extension,Json, Path, State}, http::StatusCode, routing::{get,post},
};
use serde::{Deserialize, Serialize};
use crate::app::AppState;
use catalog::{Catalog,ProductCreate,ProductId};
use tower_http::request_id::RequestId;
use crate::error::AppError;
use crate::dto::{ProductCreateRequest, ProductResponse};
use crate::observability::{self, current_request_id};
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
    Extension(raw_request_id):Extension<RequestId>,
    Json(input):Json<ProductCreateRequest>
)-> Result<(StatusCode,Json<ProductResponse>),AppError>
{
    let request_id=current_request_id(&raw_request_id);
     input
        .validate()
        .map_err(|message| AppError::validation(message, request_id))?;
    let mut catalog=state.catalog.lock().await;
    let product=catalog
    .create_product(input.into(), state.ids.as_ref(), state.clock.as_ref())
    .map_err(|err|AppError::from_catalog(err, request_id))?;
    let response=product.into();
     tracing::info!(
        request_id = %request_id,
        product_id = %product.id,
        product_handle = %product.handle,
        "product created"
    );
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
 Extension(raw_request_id): Extension<RequestId>,
 Path(id):Path<String>

)-> Result<Json<ProductResponse>,AppError>{
let request_id=current_request_id(&raw_request_id);
  let product_id: ProductId = id
        .parse()
        .map_err(|_| AppError::validation(format!("'{id}' is not a valid product id."), request_id))?;

let catalog=state.catalog.lock().await;
let product=catalog
.get_product(product_id).map_err(|err|AppError::from_catalog(err, request_id))?;

Ok(Json(product.into()))

}


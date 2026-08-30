
use axum::{
     extract::{Extension,Json, Path, State}, http::StatusCode, routing::{get,post},
};
use serde::{Deserialize, Serialize};
use crate::app::AppState;
use catalog::{Catalog,ProductCreate,ProductId};
use tower_http::request_id::RequestId;
use crate::error::AppError;
use crate::dto::{ProductCreateRequest, ProductResponse,ProductUpdateRequest};
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
   
    let product=state.catalog
    .create_product(input.into(), state.ids.as_ref(), state.clock.as_ref())
    .await
    .map_err(|err|AppError::from_catalog_db(err, request_id))?;

     tracing::info!(
        request_id = %request_id,
        product_id = %product.id,
        product_handle = %product.handle,
        "product created"
    );
    Ok((StatusCode::CREATED,Json(ProductResponse::from(&product))))
}

pub async fn list_products(
    State(state):State<AppState>,
    Extension(raw_request_id): Extension<RequestId>
)-> Result<Json<Vec<ProductResponse>>, AppError>{
     let request_id = current_request_id(&raw_request_id);

let products=state.catalog.list_products().await.map_err(|err| AppError::from_catalog_db(err, request_id))?;

    Ok(Json(products.iter().map(ProductResponse::from).collect()))
}



pub async fn get_published_products(
 State(state):State<AppState>,
 Extension(raw_request_id): Extension<RequestId>,


)-> Result<Json<Vec<ProductResponse>>,AppError>{
let request_id=current_request_id(&raw_request_id);
 

let products=state.catalog.list_published_products()
.await.map_err(|err|AppError::from_catalog_db(err, request_id))?;

Ok(Json(products.iter().map(|p|ProductResponse::from(p)).collect()))

}


pub async fn update_product(
    State(state): State<AppState>,
    Extension(raw_request_id): Extension<RequestId>,
    Path(id): Path<String>,
    Json(input): Json<ProductUpdateRequest>,
) -> Result<Json<ProductResponse>, AppError> {
    let request_id = current_request_id(&raw_request_id);

    let product_id: ProductId = id
        .parse()
        .map_err(|_| AppError::validation(format!("'{id}' is not a valid product id."), request_id))?;

    let product = state
        .catalog
        .update_product_publication(product_id, input.into(), state.clock.as_ref())
        .await
        .map_err(|err| AppError::from_catalog_db(err, request_id))?;

    tracing::info!(
        request_id = %request_id,
        product_id = %product.id,
        product_handle = %product.handle,
        published = product.published,
        "product updated"
    );

    Ok(Json(ProductResponse::from(&product)))
}

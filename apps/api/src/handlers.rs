
use axum::{
     extract::{Extension,Json, Path, State,Query}, http::StatusCode, routing::{get,post},
};
use serde::{Deserialize, Serialize};
use crate::app::AppState;
use catalog::{Catalog,ProductCreate,ProductId};
use tower_http::request_id::RequestId;
use crate::error::{AppError,ErrorEnvelope};
use crate::dto::{ProductCreateRequest, ProductResponse,ProductUpdateRequest,ImportJobView,CreateImportJobRequest,CreateImportJobResponse};
use crate::observability::{self, current_request_id};
use utoipa::ToSchema;
use crate::openapi::{HEALTH_TAG, PRODUCTS_TAG};
#[derive(Serialize,ToSchema)]
pub struct HealthResponse{
    status:&'static str
}
#[utoipa::path(
    get,
    path = "/health",
    tag = HEALTH_TAG,
    responses(
        (status = 200, description = "Service is up", body = HealthResponse)
    )
)]
pub async  fn health()->Json<HealthResponse>{
    Json(HealthResponse{
    status:"ok"
    }
    )
}

#[utoipa::path(
    post,
    path = "/api/products",
    tag = PRODUCTS_TAG,
    request_body = ProductCreateRequest,
    responses(
        (status = 201, description = "Product created", body = ProductResponse),
        (status = 400, description = "Validation failed", body = ErrorEnvelope),
        (status = 409, description = "Handle already exists", body = ErrorEnvelope)
    )
)]
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
    .create_product(ProductCreate::from(&input), state.ids.as_ref(), state.clock.as_ref())
    .await
    .map_err(|err|AppError::from_catalog_db(err, request_id))?;
     state.cache
    .delete(&cache::keys::storefront_product_page_key(&product.handle))
    .await;
     tracing::info!(
        request_id = %request_id,
        product_id = %product.id,
        product_handle = %product.handle,
        "product created"
    );
    Ok((StatusCode::CREATED,Json(ProductResponse::from(&product))))
}
#[utoipa::path(
    get,
    path = "/api/products",
    tag = PRODUCTS_TAG,
    responses(
        (status = 200, description = "All products", body = Vec<ProductResponse>),
        (status = 500, description = "Unexpected error", body = ErrorEnvelope)
    )
)]
pub async fn list_products(
    State(state):State<AppState>,
    Extension(raw_request_id): Extension<RequestId>
)-> Result<Json<Vec<ProductResponse>>, AppError>{
     let request_id = current_request_id(&raw_request_id);

let products=state.catalog.list_products().await.map_err(|err| AppError::from_catalog_db(err, request_id))?;

    Ok(Json(products.iter().map(ProductResponse::from).collect()))
}


#[utoipa::path(
    get,
    path = "/api/published_products",
    tag = PRODUCTS_TAG,
    responses(
        (status = 200, description = "Published products only", body = Vec<ProductResponse>),
         (status = 500, description = "Unexpected error", body = ErrorEnvelope)
    )
)]
pub async fn get_published_products(
 State(state):State<AppState>,
 Extension(raw_request_id): Extension<RequestId>,


)-> Result<Json<Vec<ProductResponse>>,AppError>{
let request_id=current_request_id(&raw_request_id);
 

let products=state.catalog.list_published_products()
.await.map_err(|err|AppError::from_catalog_db(err, request_id))?;

Ok(Json(products.iter().map(|p|ProductResponse::from(p)).collect()))

}

#[utoipa::path(
    patch,
    path = "/api/products/{id}",
    tag = PRODUCTS_TAG,
    params(
        ("id" = String, Path, description = "Product UUID")
    ),
    request_body = ProductUpdateRequest,
    responses(
        (status = 200, description = "Product updated", body = ProductResponse),
        (status = 400, description = "Invalid id or body", body = ErrorEnvelope),
        (status = 404, description = "Product not found", body = ErrorEnvelope)
    )
)]
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
state.cache
    .delete(&cache::keys::storefront_product_page_key(&product.handle))
    .await;
    tracing::info!(
        request_id = %request_id,
        product_id = %product.id,
        product_handle = %product.handle,
        published = product.published,
        "product updated"
    );

    Ok(Json(ProductResponse::from(&product)))
}
 use crate::openapi::IMPORT_JOBS_TAG;

 #[utoipa::path(
    post,
    path = "/api/import-jobs",
    tag = IMPORT_JOBS_TAG,
    request_body = CreateImportJobRequest,
    responses(
        (status = 202, description = "Job accepted and queued", body = CreateImportJobResponse),
        (status = 400, description = "Validation failed", body = ErrorEnvelope)
    )
)]
pub async fn create_import_job(
    State(state): State<AppState>,
    Extension(raw_request_id): Extension<RequestId>,
    Json(input): Json<CreateImportJobRequest>,
) -> Result<(StatusCode, Json<CreateImportJobResponse>), AppError> {
    let request_id = current_request_id(&raw_request_id);

    let input_path = input
        .validate()
        .map_err(|message| AppError::validation(message, request_id))?;

    let job = state
        .import_jobs
        .create(&input_path, state.ids.as_ref(), state.clock.as_ref())
        .await
        .map_err(|err| AppError::from_catalog_db(err, request_id))?;

    tracing::info!(request_id = %request_id, job_id = %job.id, status = %job.status, "import job enqueued");

    Ok((StatusCode::ACCEPTED, Json(CreateImportJobResponse {
        job: ImportJobView { id: job.id, status: job.status },
    })))
}
#[utoipa::path(
    get,
    path = "/api/import-jobs",
    tag = IMPORT_JOBS_TAG,
    params(
        ("status" = Option<String>, Query,
         description = "Filter by job status: queued, running, succeeded, failed. Defaults to failed.")
    ),
    responses(
        (status = 200, description = "Import jobs", body = Vec<ImportJobView>),
         (status = 500, description = "Unexpected error", body = ErrorEnvelope)
    )
)]

pub async fn list_import_jobs(
    State(state): State<AppState>,
    Extension(raw_request_id): Extension<RequestId>,
    Query(q): Query<ListImportJobsQuery>, // { status: Option<String> }
) -> Result<Json<Vec<ImportJobView>>, AppError> {
    let request_id = current_request_id(&raw_request_id);
    let status = q.status.as_deref().unwrap_or("failed"); // sensible default for "failed jobs are visible"
    let jobs = state.import_jobs.list_by_status(status).await
        .map_err(|err| AppError::from_catalog_db(err, request_id))?;
    Ok(Json(jobs.into_iter().map(|j| ImportJobView { id: j.id, status: j.status }).collect()))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ListImportJobsQuery {
    pub status: Option<String>,
}
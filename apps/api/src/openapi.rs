//! OpenAPI document definition (Task 14.1).
//!
//! The spec is assembled from `#[utoipa::path]`-annotated handlers via
//! `utoipa-axum`'s `OpenApiRouter`, not hand-written. That coupling is the
//! point: a handler registered in the router is the same handler that
//! contributes its path to the spec, so the two can't silently drift.
//!
//! `openapi_spec()` deliberately does NOT need an `AppState`. It builds
//! the router at the type level and throws the router away, keeping only
//! the spec -- which is what lets `make docs-api` run with no Postgres and
//! no Redis, and therefore lets `make docs-api-check` run in CI.

use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::app::AppState;
use crate::handlers::{create_product, get_published_products, health, list_products, update_product,create_import_job, list_import_jobs , __path_health,
    __path_create_product,
    __path_get_published_products,
    __path_list_products,
    __path_update_product,
    __path_create_import_job,
    __path_list_import_jobs,};


pub const HEALTH_TAG: &str = "health";
pub const PRODUCTS_TAG: &str = "products";
pub const IMPORT_JOBS_TAG: &str = "import-jobs";

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Ahlan Commerce API",
        description = "REST surface for health, products, and import jobs. \
                       The GraphQL surface is documented separately in \
                       docs/generated/schema.graphql.",
        version = "0.1.0"
    ),
    tags(
        (name = HEALTH_TAG, description = "Liveness checks"),
        (name = PRODUCTS_TAG, description = "Product catalog REST endpoints"),
        (name = IMPORT_JOBS_TAG, description = "Background product-import jobs")
    )
)]
pub struct ApiDoc;

/// Every REST route that appears in the OpenAPI spec.
///
/// Routes deliberately excluded, with reasons:
///   - `/graphql`         -- documented by schema.graphql, not OpenAPI.
///   - `/docs/scalar`     -- the docs UI itself.
///   - `/products/{handle}` -- storefront HTML, not a JSON API surface.
pub fn documented_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(health))
        .routes(routes!(list_products, create_product))
        .routes(routes!(get_published_products))
        .routes(routes!(update_product))
        .routes(routes!(create_import_job, list_import_jobs))
}

/// Builds the spec without needing a running app or any connections.
pub fn openapi_spec() -> utoipa::openapi::OpenApi {
    let (_router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(documented_router())
        .split_for_parts();
    api
}

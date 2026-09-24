use std::sync::Arc;
use axum::{
    routing::{get, post},
    Router,
};
use cache::Cache;
use catalog_db::PgImportJobs; 
use catalog::{Clock, IdGenerator, SystemClock, UuidV7Generator};
use catalog_db::{create_pool, CatalogDbError, PgCatalog};
use tower_http::cors::CorsLayer;
use crate::config::Config;
use utoipa::OpenApi;
use crate::observability;
use crate::routes;
use crate::graphql::schema::{create_schema};
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};
use crate::openapi::{documented_router, ApiDoc};
#[derive(Clone)]
pub struct AppState {
    pub catalog: PgCatalog,
    pub import_jobs: PgImportJobs,
     pub cache: Cache,
    pub config: Arc<Config>,
        
    pub ids: Arc<dyn IdGenerator>,
    pub clock: Arc<dyn Clock>,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, CatalogDbError> {
        let pool = create_pool(&config.database_url).await?;

        let catalog = PgCatalog::new(pool.clone());
        let import_jobs = PgImportJobs::new(pool.clone());

          let cache = Cache::new(&config.redis_url)
            .expect("REDIS_URL is not a valid redis connection string");
        Ok(Self {
            catalog,
            import_jobs,
            cache,
            config: Arc::new(config),
            ids: Arc::new(UuidV7Generator),
            clock: Arc::new(SystemClock),
        })
    }
}

pub fn create_router(state: AppState) -> Router {
    let schema =create_schema(state.clone());
     let (rest_router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(documented_router())
        .split_for_parts();

    let router = rest_router
        // Task 14.1: local docs route.
        .merge(Scalar::with_url(routes::DOCS_SCALAR, api))
        // Storefront HTML -- intentionally not in the OpenAPI spec.
        .route(
            routes::PRODUCT_PAGE,
            get(crate::storefront_handler::get_product_page),
        )
        // GraphQL -- documented by schema.graphql instead.
        .route("/graphql", post(crate::graphql::handler::graphql_handler))
        .layer(axum::Extension(schema))
        .layer(CorsLayer::permissive())
        .with_state(state);

    observability::with_request_tracing(router)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::ProductResponse;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::error::ErrorEnvelope;

    // ============================================================
    // TEST STATE
    // ============================================================

    async fn test_state() -> AppState {
        let url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| {
                "postgres://postgres:132456@127.0.0.1:5432/ahlan-commerce"
                    .to_string()
            });

        let pool = create_pool(&url).await
            .expect("create PostgreSQL pool for tests");

        let catalog = PgCatalog::new(pool.clone());
        let import_jobs = PgImportJobs::new(pool.clone());

        AppState {
            catalog,
            import_jobs,
            cache: Cache::new("redis://127.0.0.1:6379").expect("failed to create cache"),
            config: Arc::new(Config {
                api_bind_addr:"0.0.0.0:3000".parse().unwrap(),
                redis_url: "redis://127.0.0.1:6379".into(),
                database_url: url,
            }),
            ids: Arc::new(UuidV7Generator),
            clock: Arc::new(SystemClock),
        }
    }

    // ============================================================
    // TEST HELPERS
    // ============================================================

    fn unique_handle() -> String {
        format!("test-product-{}", Uuid::new_v4())
    }

    async fn body_json<T: serde::de::DeserializeOwned>(
        response: axum::response::Response,
    ) -> T {
        let bytes = response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes();

        serde_json::from_slice(&bytes).unwrap()
    }

    fn create_request(body: serde_json::Value) -> Request<Body> {
        Request::builder()
            .method("POST")
            .uri(routes::PRODUCTS)
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap()
    }

    // ============================================================
    // HEALTH
    // ============================================================

    #[tokio::test]
    async fn health_returns_ok() {
        let app = create_router(test_state().await);

        let response = app
            .oneshot(
                Request::builder()
                    .uri(routes::HEALTH)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    // ============================================================
    // CREATE + LIST
    // ============================================================

    #[tokio::test]
    async fn create_then_list_round_trips() {
        let app = create_router(test_state().await);

        let handle = unique_handle();

        let create_body = serde_json::json!({
            "title": "Test Product",
            "handle": handle,
            "price_cents": 1000,
            "inventory_quantity": 10,
            "published": true
        });

        // Create
        let response = app
            .clone()
            .oneshot(create_request(create_body))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let created: ProductResponse = body_json(response).await;

        assert_eq!(created.handle, handle);

        // List
        let list_response = app
            .oneshot(
                Request::builder()
                    .uri(routes::PRODUCTS)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(list_response.status(), StatusCode::OK);

        let products: Vec<ProductResponse> =
            body_json(list_response).await;

        assert!(
            products
                .iter()
                .any(|p| p.handle == handle && p.id == created.id),
            "list should contain the product just created"
        );
    }

    // ============================================================
    // VALIDATION ERROR
    // ============================================================

    #[tokio::test]
    async fn empty_title_returns_validation_failed() {
        let app = create_router(test_state().await);

        let body = serde_json::json!({
            "title": "",
            "handle": unique_handle(),
            "price_cents": 1999,
            "inventory_quantity": 5,
            "published": true
        });

        let response = app
            .oneshot(create_request(body))
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            StatusCode::BAD_REQUEST
        );

        let envelope: ErrorEnvelope =
            body_json(response).await;

        assert_eq!(
            envelope.error.code,
            "validation_failed"
        );

        assert!(
            !envelope.error.request_id.is_nil()
        );
    }

    // ============================================================
    // DUPLICATE HANDLE
    // ============================================================

    #[tokio::test]
    async fn duplicate_handle_returns_409() {
        let app = create_router(test_state().await);

        let handle = unique_handle();

        let make_body = || {
            serde_json::json!({
                "title": "T-Shirt",
                "handle": handle,
                "price_cents": 1999,
                "inventory_quantity": 5,
                "published": true
            })
        };

        // First insert
        let first = app
            .clone()
            .oneshot(create_request(make_body()))
            .await
            .unwrap();

        assert_eq!(
            first.status(),
            StatusCode::CREATED
        );

        // Second insert with same handle
        let second = app
            .oneshot(create_request(make_body()))
            .await
            .unwrap();

        assert_eq!(
            second.status(),
            StatusCode::CONFLICT
        );

        let envelope: ErrorEnvelope =
            body_json(second).await;

        assert_eq!(
            envelope.error.code,
            "duplicate_product_handle"
        );
    }
}

    // ============================================================
    // GET PRODUCT - NOT FOUND
    // ============================================================

    // #[tokio::test]
    // async fn missing_product_returns_404() {
    //     let app = create_router(test_state().await);

    //     let missing_id = Uuid::new_v4();

    //     let response = app
    //         .oneshot(
    //             Request::builder()
    //                 .uri(routes::product_url(missing_id))
    //                 .body(Body::empty())
    //                 .unwrap(),
    //         )
    //         .await
    //         .unwrap();

    //     assert_eq!(
    //         response.status(),
    //         StatusCode::NOT_FOUND
    //     );

    //     let envelope: ErrorEnvelope =
    //         body_json(response).await;

    //     assert_eq!(
    //         envelope.error.code,
    //         "not_found"
    //     );
    // }

    // ============================================================
    // MALFORMED UUID
    // ============================================================

    // #[tokio::test]
    // async fn malformed_id_returns_validation_failed_not_500() {
    //     let app = create_router(test_state().await);

    //     let response = app
    //         .oneshot(
    //             Request::builder()
    //                 .uri(routes::product_url("not-a-uuid"))
    //                 .body(Body::empty())
    //                 .unwrap(),
    //         )
    //         .await
    //         .unwrap();

    //     assert_eq!(
    //         response.status(),
    //         StatusCode::BAD_REQUEST
    //     );

    //     let envelope: ErrorEnvelope =
    //         body_json(response).await;

    //     assert_eq!(
    //         envelope.error.code,
    //         "validation_failed"
    //     );
    // }

    // ============================================================
    // REQUEST ID
    // ============================================================

//     #[tokio::test]
//     async fn error_response_request_id_matches_propagated_header() {
//         let app = create_router(test_state().await);

//         let response = app
//             .oneshot(
//                 Request::builder()
//                     .uri(routes::product_url("not-a-uuid"))
//                     .body(Body::empty())
//                     .unwrap(),
//             )
//             .await
//             .unwrap();

//         let header_request_id = response
//             .headers()
//             .get("x-request-id")
//             .expect(
//                 "PropagateRequestIdLayer should set x-request-id",
//             )
//             .to_str()
//             .unwrap()
//             .to_string();

//         let envelope: ErrorEnvelope =
//             body_json(response).await;

//         assert_eq!(
//             envelope.error.request_id.to_string(),
//             header_request_id
//         );
//     }

//     // ============================================================
//     // SUCCESS REQUEST ID
//     // ============================================================

//     #[tokio::test]
//     async fn successful_response_still_carries_a_request_id_header() {
//         let app = create_router(test_state().await);

//         let response = app
//             .oneshot(
//                 Request::builder()
//                     .uri(routes::HEALTH)
//                     .body(Body::empty())
//                     .unwrap(),
//             )
//             .await
//             .unwrap();

//         assert!(
//             response
//                 .headers()
//                 .contains_key("x-request-id")
//         );
//     }
// }
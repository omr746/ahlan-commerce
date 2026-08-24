use std::sync::Arc;
use axum::{
    extract::{State,Json},
    routing::{get,post},
    Router,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use catalog::{Clock,Catalog, IdGenerator, PgCatalog, ProductId, SystemClock, UuidV7Generator};
use crate::handlers::{create_product, get_product, health, list_products};
use crate::config::Config;
use crate::routes;
use crate::dto::{ProductCreateRequest, ProductResponse};
use crate::observability::{self, current_request_id};
#[derive(Clone)]
pub struct AppState{
    pub catalog:PgCatalog,
    pub config:Arc<Config>,
    pub ids:Arc<dyn IdGenerator>,
    pub clock:Arc<dyn Clock>
}

impl AppState{
   pub async fn new(config: Config) -> Result<Self, sqlx::Error> {
        let catalog = PgCatalog::connect(&config.database_url).await?;
        Ok(Self {
            catalog,
            config: Arc::new(config),
            ids: Arc::new(UuidV7Generator),
            clock: Arc::new(SystemClock),
        })
    }
     #[cfg(test)]
    pub fn for_tests(pool: sqlx::PgPool, config: Config) -> Self {
        Self {
            catalog: PgCatalog::from_pool(pool),
            config: Arc::new(config),
            ids: Arc::new(UuidV7Generator),
            clock: Arc::new(SystemClock),
        }
    }
}





 pub fn create_router(state:AppState)->Router{
  let router= Router::new()
   .route(routes::HEALTH,get(health))
     .route(
            routes::PRODUCTS,
            get(list_products).post(create_product),
        ).route(routes::PRODUCT_BY_ID, get(get_product))
   .with_state(state);
    observability::with_request_tracing(router)
}


#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use sqlx::postgres::PgPoolOptions;
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::error::ErrorEnvelope;

    /// These are true integration tests: they need a real Postgres
    /// reachable at `TEST_DATABASE_URL` (falls back to the same local
    /// dev database). That's expected at this stage - once storage is
    /// real, testing the HTTP layer honestly means testing it against a
    /// real database, not a mock. `packages/catalog`'s own unit tests
    /// (the in-memory `Catalog`) remain fully hermetic and need no DB.
    async fn test_pool() -> sqlx::PgPool {
        let url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:132456@127.0.0.1:5432/ahlan-commerce".to_string());
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await
            .expect("connect to Postgres for tests - see README's 'Running tests' section");
        catalog::PgCatalog::ensure_schema(&pool)
            .await
            .expect("ensure products table exists");
        pool
    }

    async fn test_state() -> AppState {
        let pool = test_pool().await;
        AppState::for_tests(
            pool,
            Config { host: "127.0.0.1".into(), port: 3000, database_url: String::new() },
        )
    }

    /// A fresh handle per call, not a shared constant - tests may run
    /// concurrently against the same physical database/table (no
    /// per-test transaction rollback at this stage), so every test needs
    /// data that can't collide with any other test's data.
    fn unique_handle() -> String {
        format!("test-product-{}", Uuid::new_v4())
    }

    async fn body_json<T: serde::de::DeserializeOwned>(response: axum::response::Response) -> T {
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
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

    #[tokio::test]
    async fn health_returns_ok() {
        let app = create_router(test_state().await);
        let response = app
            .oneshot(Request::builder().uri(routes::HEALTH).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

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

        let response = app.clone().oneshot(create_request(create_body)).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
        let created: ProductResponse = body_json(response).await;
        assert_eq!(created.handle, handle);

        let list_response = app
            .oneshot(Request::builder().uri(routes::PRODUCTS).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(list_response.status(), StatusCode::OK);

        let products: Vec<ProductResponse> = body_json(list_response).await;
        assert!(
            products.iter().any(|p| p.handle == handle && p.id == created.id),
            "list should contain the product just created"
        );
    }

    // --- Task 03A.3 / 03B: error mapping, asserted by status/code/shape
    // only, never by internal error text. Still true now that the
    // "not found"/"duplicate" facts come from a real database instead of
    // a Vec. ---

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

        let response = app.oneshot(create_request(body)).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let envelope: ErrorEnvelope = body_json(response).await;
        assert_eq!(envelope.error.code, "validation_failed");
        assert!(!envelope.error.request_id.is_nil());
    }

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

        let first = app.clone().oneshot(create_request(make_body())).await.unwrap();
        assert_eq!(first.status(), StatusCode::CREATED);

        // Postgres's `unique` constraint on `handle` is what actually
        // rejects this second insert - this test is exercising the real
        // constraint, not a Rust-side duplicate check.
        let second = app.oneshot(create_request(make_body())).await.unwrap();
        assert_eq!(second.status(), StatusCode::CONFLICT);

        let envelope: ErrorEnvelope = body_json(second).await;
        assert_eq!(envelope.error.code, "duplicate_product_handle");
    }

    #[tokio::test]
    async fn missing_product_returns_404() {
        let app = create_router(test_state().await);
        // A fresh v4 id: the app only ever inserts v7 ids, so this is
        // guaranteed to never exist regardless of what other tests have
        // inserted.
        let missing_id = Uuid::new_v4();

        let response = app
            .oneshot(
                Request::builder()
                    .uri(routes::product_url(missing_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let envelope: ErrorEnvelope = body_json(response).await;
        assert_eq!(envelope.error.code, "not_found");
    }

    #[tokio::test]
    async fn malformed_id_returns_validation_failed_not_500() {
        let app = create_router(test_state().await);

        let response = app
            .oneshot(
                Request::builder()
                    .uri(routes::product_url("not-a-uuid"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let envelope: ErrorEnvelope = body_json(response).await;
        assert_eq!(envelope.error.code, "validation_failed");
    }

    // --- Task 03B: the request-id header set by the tracing layer must
    // match the request_id inside the error envelope. ---

    #[tokio::test]
    async fn error_response_request_id_matches_propagated_header() {
        let app = create_router(test_state().await);

        let response = app
            .oneshot(
                Request::builder()
                    .uri(routes::product_url("not-a-uuid"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let header_request_id = response
            .headers()
            .get("x-request-id")
            .expect("PropagateRequestIdLayer should set x-request-id")
            .to_str()
            .unwrap()
            .to_string();

        let envelope: ErrorEnvelope = body_json(response).await;
        assert_eq!(envelope.error.request_id.to_string(), header_request_id);
    }

    #[tokio::test]
    async fn successful_response_still_carries_a_request_id_header() {
        let app = create_router(test_state().await);
        let response = app
            .oneshot(Request::builder().uri(routes::HEALTH).body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert!(response.headers().contains_key("x-request-id"));
    }
}

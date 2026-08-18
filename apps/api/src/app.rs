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
use crate::handlers::{create_product, get_product, health, list_products};
use crate::config::Config;
use crate::routes;
use crate::dto::{ProductCreateRequest, ProductResponse};

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
      #[cfg(test)]
    pub fn with_fixed(config: Config, ids: Arc<dyn IdGenerator>, clock: Arc<dyn Clock>) -> Self {
        Self {
            catalog: Arc::new(Mutex::new(Catalog::new())),
            config: Arc::new(config),
            ids,
            clock,
        }
    }
}





 pub fn create_router(state:AppState)->Router{
   Router::new()
   .route(routes::HEALTH,get(health))
     .route(
            routes::PRODUCTS,
            get(list_products).post(create_product),
        ).route(routes::PRODUCT_BY_ID, get(get_product))
   .with_state(state)
}


#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use catalog::{FixedClock, FixedIdGenerator};
    use chrono::{TimeZone, Utc};
    use http_body_util::BodyExt;
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::error::ErrorEnvelope;

    fn test_state() -> AppState {
        let id = Uuid::parse_str("018f5a3e-0000-7000-8000-000000000001").unwrap();
        let ts = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
        AppState::with_fixed(
            Config { host: "127.0.0.1".into(), port: 3000 },
            Arc::new(FixedIdGenerator(id)),
            Arc::new(FixedClock(ts)),
        )
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
        let app = create_router(test_state());
        let response = app
            .oneshot(Request::builder().uri(routes::HEALTH).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_then_list_round_trips() {
        let app = create_router(test_state());

        let create_body = serde_json::json!({
            "title": "Test Product",
            "handle": "test-product",
            "price_cents": 1000,
            "inventory_quantity": 10,
            "published": true
        });

        let response = app.clone().oneshot(create_request(create_body)).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let list_response = app
            .oneshot(Request::builder().uri(routes::PRODUCTS).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(list_response.status(), StatusCode::OK);

        let products: Vec<ProductResponse> = body_json(list_response).await;
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].title, "Test Product");
    }

    // --- Task 03A.3: error mapping, asserted by status/code/shape only,
    // never by internal error text - the whole point of the envelope. ---

    #[tokio::test]
    async fn empty_title_returns_validation_failed() {
        let app = create_router(test_state());
        let body = serde_json::json!({
            "title": "",
            "handle": "t-shirt",
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
        let app = create_router(test_state());
        let make_body = || {
            serde_json::json!({
                "title": "T-Shirt",
                "handle": "t-shirt",
                "price_cents": 1999,
                "inventory_quantity": 5,
                "published": true
            })
        };

        let first = app.clone().oneshot(create_request(make_body())).await.unwrap();
        assert_eq!(first.status(), StatusCode::CREATED);

        let second = app.oneshot(create_request(make_body())).await.unwrap();
        assert_eq!(second.status(), StatusCode::CONFLICT);

        let envelope: ErrorEnvelope = body_json(second).await;
        assert_eq!(envelope.error.code, "duplicate_product_handle");
    }

    #[tokio::test]
    async fn missing_product_returns_404() {
        let app = create_router(test_state());
        let missing_id = "018f5a3e-0000-7000-8000-0000000000ff";

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
        let app = create_router(test_state());

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
}

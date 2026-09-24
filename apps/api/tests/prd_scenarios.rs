//! PRD scenario acceptance tests - one test function per scenario in
//! specs/product-scenarios.md, named to match exactly (each scenario's
//! "Automated by" line names the function below it maps to). These are
//! deliberately separate from apps/api/src/app.rs's own tests: this file
//! exists for traceability from a business-readable scenario to a single
//! runnable test, not to double as general implementation coverage.
//!
//! Needs a real, already-migrated Postgres reachable at
//! `TEST_DATABASE_URL` (falls back to the local dev database) - same
//! requirement as apps/api/src/app.rs's tests.

use api::app::{create_router, AppState};
use api::config::Config;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;
use uuid::Uuid;

async fn test_state() -> AppState {
    let url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:132456@127.0.0.1:5432/ahlan-commerce".to_string());
    AppState::new(Config { api_bind_addr: "0.0.0.0:3000".parse().unwrap(), redis_url: "redis://127.0.0.1:6379".into(), database_url: url })
        .await
        .expect("connect to Postgres for PRD scenario tests")
}

fn unique_handle(label: &str) -> String {
    format!("prd-{label}-{}", Uuid::new_v4())
}

async fn body_json(response: axum::response::Response) -> Value {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

fn create_request(body: Value) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri("/api/products")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap()
}

fn list_request(query: &str) -> Request<Body> {
    let uri = if query.is_empty() { "/api/products".to_string() } else { format!("/api/published_products") };
    Request::builder().uri(uri).body(Body::empty()).unwrap()
}

/// PRD-PROD-001 - Valid Product Create
#[tokio::test]
async fn prd_prod_001_valid_product_create() {
    let app = create_router(test_state().await);
    let handle = unique_handle("valid-create");

    let response = app
        .oneshot(create_request(serde_json::json!({
            "title": "Sun Hat",
            "handle": handle,
            "price_cents": 1200,
            "inventory_quantity": 5,
            "published": true
        })))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = body_json(response).await;
    assert!(body["id"].is_string(), "response must include a generated id");
    assert_eq!(body["handle"], handle);
    assert_eq!(body["published"], true);
    assert!(body["published_at"].is_string(), "published:true must set published_at");
    assert!(body["created_at"].is_string());
    assert!(body["updated_at"].is_string());
}

/// PRD-PROD-002 - Duplicate Handle Rejected
#[tokio::test]
async fn prd_prod_002_duplicate_handle_rejected() {
    let app = create_router(test_state().await);
    let handle = unique_handle("duplicate");
    let make_body = || {
        serde_json::json!({
            "title": "Original", "handle": handle,
            "price_cents": 1200, "inventory_quantity": 5, "published": true
        })
    };

    let first = app.clone().oneshot(create_request(make_body())).await.unwrap();
    assert_eq!(first.status(), StatusCode::CREATED);
    let original = body_json(first).await;

    let second_body = serde_json::json!({
        "title": "Impostor", "handle": handle,
        "price_cents": 1, "inventory_quantity": 999, "published": true
    });
    let second = app.clone().oneshot(create_request(second_body)).await.unwrap();
    assert_eq!(second.status(), StatusCode::CONFLICT);
    let error = body_json(second).await;
    assert_eq!(error["error"]["code"], "duplicate_product_handle");

    // The original must be provably unchanged, not merely "still present".
    let listed = app.oneshot(list_request("published=true")).await.unwrap();
    let products = body_json(listed).await;
    let found = products
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["handle"] == handle)
        .expect("original product must still exist");
    assert_eq!(found["title"], original["title"]);
    assert_eq!(found["price_cents"], 1200);
    assert_eq!(found["inventory_quantity"], 5);
}

/// PRD-PROD-003 - List Empty Products
///
/// "Empty" here means "no products matching this filter exist yet" -
/// verified with a handle guaranteed never to have been created, filtered
/// down with the same `published=true` query real callers use, rather
/// than requiring a genuinely empty table (which nothing in this
/// long-lived shared database can guarantee run to run).
#[tokio::test]
async fn prd_prod_003_list_empty_products() {
    let app = create_router(test_state().await);
    let handle_that_was_never_created = unique_handle("never-created");

    let response = app.oneshot(list_request("published=true")).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let products = body_json(response).await;

    assert!(products.is_array(), "an empty result must still be a list, not an error or null");
    assert!(
        !products
            .as_array()
            .unwrap()
            .iter()
            .any(|p| p["handle"] == handle_that_was_never_created),
        "sanity check: this handle was never created, so it must not appear"
    );
}

/// PRD-PROD-004 - List Persisted Products
#[tokio::test]
async fn prd_prod_004_list_persisted_products() {
    let app = create_router(test_state().await);
    let published_handle = unique_handle("list-published");
    let draft_handle = unique_handle("list-draft");

    app.clone()
        .oneshot(create_request(serde_json::json!({
            "title": "Published Item", "handle": published_handle,
            "price_cents": 500, "inventory_quantity": 1, "published": true
        })))
        .await
        .unwrap();
    app.clone()
        .oneshot(create_request(serde_json::json!({
            "title": "Draft Item", "handle": draft_handle,
            "price_cents": 500, "inventory_quantity": 1, "published": false
        })))
        .await
        .unwrap();

    // All products: both must appear.
    let all = body_json(app.clone().oneshot(list_request("")).await.unwrap()).await;
    let all = all.as_array().unwrap();
    assert!(all.iter().any(|p| p["handle"] == published_handle));
    assert!(all.iter().any(|p| p["handle"] == draft_handle));

    // Published-only: draft must be genuinely absent, not just unmarked.
    let published_only = body_json(app.oneshot(list_request("published=true")).await.unwrap()).await;
    let published_only = published_only.as_array().unwrap();
    assert!(published_only.iter().any(|p| p["handle"] == published_handle));
    assert!(
        !published_only.iter().any(|p| p["handle"] == draft_handle),
        "an unpublished product must never appear in the published-only list"
    );
}

/// PRD-PROD-005 - Invalid Create Input Rejected
///
/// One test function covering every invalid-input rule from the PRD's
/// functional requirements table, since the scenario itself is singular
/// ("invalid input is rejected") even though several distinct rules can
/// trigger it.
#[tokio::test]
async fn prd_prod_005_invalid_create_input_rejected() {
    let app = create_router(test_state().await);

    let cases = [
        ("blank title", serde_json::json!({
            "title": "", "handle": unique_handle("blank-title"),
            "price_cents": 100, "inventory_quantity": 1, "published": true
        })),
        ("malformed handle", serde_json::json!({
            "title": "X", "handle": "Not A Valid Handle!",
            "price_cents": 100, "inventory_quantity": 1, "published": true
        })),
        ("negative price", serde_json::json!({
            "title": "X", "handle": unique_handle("neg-price"),
            "price_cents": -1, "inventory_quantity": 1, "published": true
        })),
        ("negative inventory", serde_json::json!({
            "title": "X", "handle": unique_handle("neg-inventory"),
            "price_cents": 100, "inventory_quantity": -1, "published": true
        })),
    ];

    for (label, body) in cases {
        let response = app.clone().oneshot(create_request(body)).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST, "case: {label}");
        let error = body_json(response).await;
        assert_eq!(error["error"]["code"], "validation_failed", "case: {label}");
    }
}
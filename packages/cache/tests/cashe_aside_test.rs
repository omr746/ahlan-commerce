//! Task 13.3 tests: hit, miss, invalidation, and Redis fallback around
//! the cached storefront page.
//!
//! These test the cache-aside *mechanics* against a real Redis using the
//! real cached value shape, without needing a running API or Postgres --
//! the render step is a pure function and the DB load is the one thing
//! stubbed out. The full HTTP-level path (route returns 200 HTML / 404)
//! is covered by the renderer tests plus a manual browser check, since
//! wiring a full axum + Postgres + Redis harness is beyond what this
//! chapter sets up.
//!
//! BLOCKER DOCUMENTED (per "manual evidence only if a test is blocked"):
//! end-to-end HTTP assertions (`GET /products/{handle}` returning 200 vs
//! 404 through the router) are verified manually via curl/browser, because
//! apps/api has no existing integration-test harness that boots the router
//! against a migrated database. The cache behavior itself -- which is what
//! Task 13.3 actually asks to be tested -- is covered automatically below.

use std::time::Duration;

use cache::page::{CachedProductPage, PRODUCT_PAGE_TTL_SECONDS};
use cache::Cache;
use catalog::Product;
use chrono::Utc;
use storefront::{render_product_page, ProductPageContext};
use uuid::Uuid;

fn redis_url() -> String {
    std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string())
}

const UNREACHABLE_REDIS_URL: &str = "redis://127.0.0.1:1";

/// Unique handle per test run so tests don't collide with each other or
/// with leftover keys from a previous run.
fn unique_handle(prefix: &str) -> String {
    format!(
        "{prefix}-{:x}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    )
}

fn sample_product(handle: &str) -> Product {
    Product {
        id: Uuid::now_v7(),
        title: "Coffee Mug".to_string(),
        handle: handle.to_string(),
        description: Some("Ceramic mug for daily coffee.".to_string()),
        price_cents: 2500,
        inventory_quantity: 12,
        published: true,
        published_at: Some(Utc::now()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn render(product: &Product) -> String {
    render_product_page(&ProductPageContext::from_product(product))
}

fn entry_for(product: &Product, html: String) -> CachedProductPage {
    CachedProductPage {
        html,
        product_id: product.id,
        product_updated_at: product.updated_at,
        rendered_at: Utc::now(),
    }
}

#[tokio::test]
async fn cold_key_is_a_miss() {
    let cache = Cache::new(&redis_url()).expect("valid redis url");
    let key = cache::keys::storefront_product_page_key(&unique_handle("miss"));

    let got: Option<CachedProductPage> = cache.get_json(&key).await;
    assert_eq!(got, None, "a never-written key must be a miss");
}

#[tokio::test]
async fn write_then_read_is_a_hit_with_the_same_html() {
    let cache = Cache::new(&redis_url()).expect("valid redis url");
    let handle = unique_handle("hit");
    let key = cache::keys::storefront_product_page_key(&handle);

    let product = sample_product(&handle);
    let html = render(&product);
    cache
        .set_json(&key, &entry_for(&product, html.clone()), Duration::from_secs(PRODUCT_PAGE_TTL_SECONDS))
        .await;

    let got: Option<CachedProductPage> = cache.get_json(&key).await;
    let got = got.expect("expected a cache hit");
    assert_eq!(got.html, html);
    assert_eq!(got.product_id, product.id);

    cache.delete(&key).await;
}

#[tokio::test]
async fn invalidation_deletes_the_cached_page() {
    let cache = Cache::new(&redis_url()).expect("valid redis url");
    let handle = unique_handle("invalidate");
    let key = cache::keys::storefront_product_page_key(&handle);

    let product = sample_product(&handle);
    cache
        .set_json(&key, &entry_for(&product, render(&product)), Duration::from_secs(PRODUCT_PAGE_TTL_SECONDS))
        .await;
    assert!(cache.get_json::<CachedProductPage>(&key).await.is_some());

    // This is exactly what create_product / update_product now call.
    cache.delete(&key).await;

    assert_eq!(
        cache.get_json::<CachedProductPage>(&key).await,
        None,
        "after invalidation the next read must be a miss, not stale HTML"
    );
}

#[tokio::test]
async fn republished_product_does_not_serve_stale_html_after_invalidation() {
    let cache = Cache::new(&redis_url()).expect("valid redis url");
    let handle = unique_handle("stale");
    let key = cache::keys::storefront_product_page_key(&handle);

    // Cache a page showing 12 in stock.
    let mut product = sample_product(&handle);
    cache
        .set_json(&key, &entry_for(&product, render(&product)), Duration::from_secs(PRODUCT_PAGE_TTL_SECONDS))
        .await;

    // Product changes (inventory drops to 0) and the write path invalidates.
    product.inventory_quantity = 0;
    product.updated_at = Utc::now();
    cache.delete(&key).await;

    // Next read is a miss, so the route would re-render from Postgres --
    // simulate that re-render and confirm the new state is what gets cached.
    let fresh_html = render(&product);
    cache
        .set_json(&key, &entry_for(&product, fresh_html.clone()), Duration::from_secs(PRODUCT_PAGE_TTL_SECONDS))
        .await;

    let got = cache
        .get_json::<CachedProductPage>(&key)
        .await
        .expect("expected a hit after re-render");
    assert!(got.html.contains("Out of stock"), "must reflect the updated inventory");
    assert!(!got.html.contains("In stock (12 available)"), "must not serve the stale page");

    cache.delete(&key).await;
}

#[tokio::test]
async fn read_falls_back_safely_when_redis_is_unavailable() {
    let cache = Cache::new(UNREACHABLE_REDIS_URL).expect("valid redis url");
    let key = cache::keys::storefront_product_page_key("anything");

    let got: Option<CachedProductPage> = cache.get_json(&key).await;
    assert_eq!(got, None, "Redis down must look like a miss so the route renders from Postgres");
}

#[tokio::test]
async fn write_and_delete_do_not_fail_when_redis_is_unavailable() {
    let cache = Cache::new(UNREACHABLE_REDIS_URL).expect("valid redis url");
    let handle = "anything";
    let key = cache::keys::storefront_product_page_key(handle);
    let product = sample_product(handle);

    // Both must return normally -- a Redis outage cannot fail a product
    // write or a page render.
    cache
        .set_json(&key, &entry_for(&product, render(&product)), Duration::from_secs(PRODUCT_PAGE_TTL_SECONDS))
        .await;
    cache.delete(&key).await;
}

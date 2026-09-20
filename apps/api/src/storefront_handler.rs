//! Storefront product page route (Task 13.1) and the cache-aside read
//! path around it (Task 13.2).
//!
//! Division of responsibility, which is the thing Task 13.2 asks you to be
//! able to explain:
//!
//!   - LOADS DATA:    `state.catalog.get_published_product_by_handle(..)`
//!                    (packages/catalog-db) -- the only thing that talks
//!                    to PostgreSQL.
//!   - RENDERS HTML:  `storefront::ProductPageContext::from_product` +
//!                    `storefront::render_product_page` -- pure functions,
//!                    no I/O, no awareness that a cache exists.
//!   - OWNS FRESHNESS: this file. It decides when to trust Redis, when to
//!                    fall through to Postgres, what TTL to write, and
//!                    (with handlers.rs) when to invalidate.
//!
//! Nothing below can fail the request because of Redis. Every cache
//! operation is best-effort; a total Redis outage degrades this route to
//! "always render from Postgres", which is exactly the pre-cache behavior.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::Extension;
use cache::page::{CachedProductPage, PRODUCT_PAGE_TTL_SECONDS};
use chrono::Utc;
use std::time::Duration;
use storefront::{render_product_page, ProductPageContext};
use tower_http::request_id::RequestId;

use crate::app::AppState;
use crate::error::AppError;
use crate::observability::current_request_id;

pub async fn get_product_page(
    State(state): State<AppState>,
    Extension(raw_request_id): Extension<RequestId>,
    Path(handle): Path<String>,
) -> Result<Response, AppError> {
    let request_id = current_request_id(&raw_request_id);
    let cache_key = cache::keys::storefront_product_page_key(&handle);

    // --- Read behavior step 1 & 2: try Redis, return cached HTML on hit.
    //
    // A miss, a Redis error, and invalid/stale-shaped JSON are all
    // indistinguishable here by design -- `get_json` already collapsed
    // them into `None` and logged which one it was with `cache_key`.
    if let Some(cached) = state
        .cache
        .get_json::<CachedProductPage>(&cache_key)
        .await
    {
        return Ok(Html(cached.html).into_response());
    }

    // --- Step 3: load from PostgreSQL and render.
    //
    // A product that doesn't exist and a product that exists but is
    // unpublished are the same 404 to a storefront visitor -- the route
    // contract deliberately doesn't distinguish them, so an unpublished
    // product's existence isn't leaked.
    let product = state
        .catalog
        .get_published_product_by_handle(&handle)
        .await
        .map_err(|err| AppError::from_catalog_db(err, request_id))?;

    let Some(product) = product else {
        // Deliberately NOT cached. A 404 now can become a 200 the moment
        // someone publishes this handle, and caching the negative result
        // would serve a stale 404 for up to the full TTL.
        return Ok((StatusCode::NOT_FOUND, Html(render_not_found())).into_response());
    };

    let context = ProductPageContext::from_product(&product);
    let html = render_product_page(&context);

    // --- Step 4: try to populate the cache. Step 5: a failure here is
    // logged inside `set_json` and changes nothing about what we return.
    let entry = CachedProductPage {
        html: html.clone(),
        product_id: product.id,
        product_updated_at: product.updated_at,
        rendered_at: Utc::now(),
    };
    state
        .cache
        .set_json(&cache_key, &entry, Duration::from_secs(PRODUCT_PAGE_TTL_SECONDS))
        .await;

    Ok(Html(html).into_response())
}

fn render_not_found() -> String {
    "<!doctype html>\n\
     <html lang=\"en\">\n\
     <head><meta charset=\"utf-8\"><title>Product not found</title></head>\n\
     <body><main><h1>Product not found</h1></main></body>\n\
     </html>\n"
        .to_string()
}

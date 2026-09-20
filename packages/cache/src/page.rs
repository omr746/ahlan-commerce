//! The exact JSON value shape stored in Redis under
//! `storefront:product-page:{handle}`, per cache-contract.md.
//!
//! `product_updated_at` and `rendered_at` aren't read by the cache-aside
//! path itself (TTL + explicit invalidation handle freshness). They're
//! stored because the contract specifies them, and they make a cached
//! entry self-describing when debugging staleness: you can tell how old
//! the render is and which version of the product it came from.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// TTL for rendered storefront product pages, per cache-contract.md.
/// This is a safety net, not the primary freshness mechanism -- explicit
/// invalidation on product writes is (see Task 13.3).
pub const PRODUCT_PAGE_TTL_SECONDS: u64 = 300;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CachedProductPage {
    pub html: String,
    pub product_id: Uuid,
    pub product_updated_at: DateTime<Utc>,
    pub rendered_at: DateTime<Utc>,
}

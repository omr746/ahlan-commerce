
pub const HEALTH:&str="/health";
pub const PRODUCTS:&str="/api/products";
pub const PUBLISHED_PRODUCTS:&str="/api/published_products";

pub const PRODUCT_BY_ID: &str = "/api/products/{id}";

pub fn product_url(id: impl std::fmt::Display) -> String {
    format!("/api/products/{id}")
}

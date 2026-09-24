pub const HEALTH: &str = "/health";
pub const PRODUCTS: &str = "/api/products";
pub const PUBLISHED_PRODUCTS: &str = "/api/published_products";
pub const IMPORT_JOBS: &str = "/api/import-jobs";
pub const IMPORT_JOB_BY_ID: &str = "/api/import-jobs/{id}";
pub const PRODUCT_BY_ID: &str = "/api/products/{id}";
pub const PRODUCT_PAGE: &str = "/products/{handle}";
pub const DOCS_SCALAR: &str = "/docs/scalar";
pub fn product_url(id: impl std::fmt::Display) -> String {
    format!("/api/products/{id}")
}
pub fn import_job_url(id: impl std::fmt::Display) -> String {
    format!("/api/import-jobs/{id}")
}

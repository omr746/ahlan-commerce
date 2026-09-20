use api::dto::ProductCreateRequest;
use api::error::AppError;
use catalog::{Clock, IdGenerator,ProductCreate};
use catalog_db::error::CatalogDbError;
use catalog_db::PgCatalog;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct ImportFile {
    pub products: Vec<ProductCreateRequest>,
}

/// Reads, validates (same ProductCreateRequest::validate the API uses),
/// and creates products one at a time via catalog.create_product. Stops at
/// the first failure. Every error path goes through AppError, same as the
/// HTTP handlers, so error shape/logging is identical between the two.
pub async fn import_file(
    catalog: &PgCatalog,
    ids: &dyn IdGenerator,
    clock: &dyn Clock,
    input_path: &str,
    request_id: Uuid,
) -> Result<usize, AppError> {
    let raw = tokio::fs::read_to_string(input_path)
        .await
        .map_err(|_| AppError::validation(format!("input file not found: {input_path}"), request_id))?;

    let file: ImportFile = serde_json::from_str(&raw)
        .map_err(|_| AppError::validation("input file is not valid JSON".to_string(), request_id))?;

    let mut created = 0usize;
    for (i, product) in file.products.iter().enumerate() {
        product
            .validate()
            .map_err(|msg| AppError::validation(format!("product[{i}]: {msg}"), request_id))?;

        match catalog.create_product(ProductCreate::from(product), ids, clock).await {
            Ok(_) => created += 1,
            Err(err) => return Err(AppError::from_catalog_db(err, request_id)),
        }
    }

    Ok(created)
}
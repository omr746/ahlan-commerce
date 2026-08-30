use serde::{Deserialize, Serialize};

use catalog::{Product, ProductCreate, ProductId,ProductUpdate};
use chrono::{DateTime, Utc};



#[derive(Debug, Deserialize)]
pub struct ProductUpdateRequest {
    #[serde(default)]
    pub description: Option<String>,
    pub published: bool,
}

impl From<ProductUpdateRequest> for ProductUpdate {
    fn from(req: ProductUpdateRequest) -> Self {
        ProductUpdate { description: req.description, published: req.published }
    }
}

#[derive(Debug, Deserialize)]
pub struct ProductCreateRequest{
     pub title: String,
    pub handle: String,
    pub price_cents: i64,
    pub inventory_quantity: i64,
    pub published: bool,
    #[serde(default)]
    pub description: Option<String>

}
impl ProductCreateRequest {
 
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("Product title is required.".to_string());
        }
        if self.handle.trim().is_empty() {
            return Err("Product handle is required.".to_string());
        }
        if !is_valid_handle(&self.handle) {
            return Err(
                "Product handle must be lowercase letters, numbers, and hyphens only.".to_string(),
            );
        }
        if self.price_cents < 0 {
            return Err("price_cents must be greater than or equal to 0.".to_string());
        }
        if self.price_cents > i32::MAX as i64 {
            
            return Err("price_cents is too large.".to_string());
        }
        if self.inventory_quantity < 0 {
            return Err("inventory_quantity must be greater than or equal to 0.".to_string());
        }
        if self.inventory_quantity > i32::MAX as i64 {
            return Err("inventory_quantity is too large.".to_string());
        }
        Ok(())
    }
}
fn is_valid_handle(handle: &str) -> bool {
    !handle.is_empty()
        && !handle.starts_with('-')
        && !handle.ends_with('-')
        && handle
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}
impl From<ProductCreateRequest> for ProductCreate{
    fn from(req:ProductCreateRequest)->Self{
        ProductCreate {
            title: req.title,
            handle: req.handle,
            price_cents: req.price_cents as u32,
            inventory_quantity: req.inventory_quantity as u32,
            published: req.published,
            description:req.description
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ProductResponse {
     pub id: ProductId,
    pub title: String,
    pub handle: String,
    pub description: Option<String>,
    pub price_cents: u32,
    pub inventory_quantity: u32,
    pub published: bool,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
    


impl From<&Product> for ProductResponse {
    fn from(p: &Product) -> Self {
        Self {
            id: p.id,
            title: p.title.clone(),
            handle: p.handle.clone(),
            price_cents: p.price_cents,
            inventory_quantity: p.inventory_quantity,
            published: p.published,
            created_at:p.created_at,
            updated_at:p.updated_at,
            description:p.description.clone(),
            published_at:p.published_at

           
        }
    }
}

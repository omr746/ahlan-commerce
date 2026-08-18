use serde::{Deserialize, Serialize};

use catalog::{Product, ProductCreate, ProductId};
use chrono::{DateTime, Utc};


#[derive(Debug, Deserialize)]
pub struct ProductCreateRequest{
     pub title: String,
    pub handle: String,
    pub price_cents: u32,
    pub inventory_quantity: u32,
    pub published: bool,

}
impl ProductCreateRequest {
 
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("Product title is required.".to_string());
        }
        if self.handle.trim().is_empty() {
            return Err("Product handle is required.".to_string());
        }
        Ok(())
    }
}
impl From<ProductCreateRequest> for ProductCreate{
    fn from(req:ProductCreateRequest)->Self{
        ProductCreate {
            title: req.title,
            handle: req.handle,
            price_cents: req.price_cents,
            inventory_quantity: req.inventory_quantity,
            published: req.published,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ProductResponse {
    pub id: ProductId,
    pub title: String,
    pub handle: String,
    pub price_cents: u32,
    pub inventory_quantity: u32,
    pub published: bool,
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
            updated_at:p.updated_at

           
        }
    }
}

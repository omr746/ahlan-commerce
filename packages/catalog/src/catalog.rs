use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::clock::Clock;
use crate::id::{IdGenerator, ProductId};
use crate::error::CatalogError;
#[derive(Debug)]
pub struct Product{
    pub id:ProductId,
    pub title:String,
    pub handle:String,
    pub price_cents:u32,
    pub inventory_quantity:u32,
    pub published:bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct ProductCreate{
    pub title:String,
    pub handle:String,
    pub price_cents:u32,
    pub inventory_quantity:u32,
    pub published:bool
}
#[derive(Debug)]
pub struct Catalog{
    pub products:Vec<Product>
}
impl Catalog{
     pub fn new() -> Self {
        Self {
            products: Vec::new(),
        }
    }
    pub fn create_product(&mut self, input:ProductCreate,id:&dyn IdGenerator,clock:&dyn Clock)
    ->Result<&Product,CatalogError>{
        if self.products.iter().any(|p| p.handle == input.handle) {
            return Err(CatalogError::DuplicateHandle(input.handle));
        }
        let now = clock.now();
        let product=Product{
            id:id.new_id(),
            title:input.title,
            handle:input.handle,
            price_cents:input.price_cents,
            inventory_quantity:input.inventory_quantity,
            published:input.published,
            created_at: now,
            updated_at: now,

        };
        self.products.push(product);
        Ok(self.products.last().unwrap())
    }
    pub fn list_products(&self)->&Vec<Product>{
       &self.products
    }
    pub fn get_product(&self,id:ProductId)-> Result<&Product,CatalogError>{
      self.products.iter().find(|p|p.id==id).ok_or(CatalogError::NotFound(id))
    }
}

#[cfg(test)]
mod tests{
     use super::*;
    use crate::id::FixedIdGenerator;
    use crate::clock::FixedClock;
    use chrono::TimeZone;
    use uuid::Uuid;

    fn fixture() -> (FixedIdGenerator, FixedClock) {
        let id = Uuid::parse_str("018f5a3e-0000-7000-8000-000000000001").unwrap();
        let ts = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
        (FixedIdGenerator(id), FixedClock(ts))
    }
    #[test]
    fn create_product_test(){
        let (ids, clock) = fixture();
        let product=ProductCreate{
            title:"Test Product".to_string(),
            handle:"test-product".to_string(),
            price_cents:1000,
            inventory_quantity:10,
            published:true
        };
        let mut catalog=Catalog::new();
        let created_product=catalog.create_product(product,&ids,&clock).unwrap();
        assert_eq!(created_product.title,"Test Product");
        assert_eq!(created_product.handle,"test-product");
        assert_eq!(created_product.price_cents,1000);
        assert_eq!(created_product.inventory_quantity,10);
        assert_eq!(created_product.published,true);

    }
    #[test]
    fn list_products_test(){    
         let (ids, clock) = fixture();
        let product=ProductCreate{
            title:"Test Product".to_string(),
            handle:"test-product".to_string(),
            price_cents:1000,
            inventory_quantity:10,
            published:true
        };
        let mut catalog=Catalog{products:Vec::new()};
        catalog.create_product(product,&ids,&clock);
        let products=catalog.list_products();
        assert_eq!(products.len(),1);
        assert_eq!(products[0].title,"Test Product");
    }


}


pub type ProductId=u32;
#[derive(Debug)]
pub struct Product{
    pub id:ProductId,
    pub title:String,
    pub handle:String,
    pub price_cents:u32,
    pub inventory_quantity:u32,
    pub published:bool
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
    pub fn create_product(&mut self, input:ProductCreate)->&Product{
        let product=Product{
            id:1,
            title:input.title,
            handle:input.handle,
            price_cents:input.price_cents,
            inventory_quantity:input.inventory_quantity,
            published:input.published
        };
        self.products.push(product);
        self.products.last().unwrap()
    }
    pub fn list_products(&self)-> &Vec<Product>{
        &self.products
    }
}

#[cfg(test)]
mod tests{
     use super::*;

    #[test]
    fn create_product_test(){
        let product=ProductCreate{
            title:"Test Product".to_string(),
            handle:"test-product".to_string(),
            price_cents:1000,
            inventory_quantity:10,
            published:true
        };
        let mut catalog=Catalog::new();
        let created_product=catalog.create_product(product);
        assert_eq!(created_product.title,"Test Product");
        assert_eq!(created_product.handle,"test-product");
        assert_eq!(created_product.price_cents,1000);
        assert_eq!(created_product.inventory_quantity,10);
        assert_eq!(created_product.published,true);

    }
    #[test]
    fn list_products_test(){    
        let product=ProductCreate{
            title:"Test Product".to_string(),
            handle:"test-product".to_string(),
            price_cents:1000,
            inventory_quantity:10,
            published:true
        };
        let mut catalog=Catalog{products:Vec::new()};
        catalog.create_product(product);
        let products=catalog.list_products();
        assert_eq!(products.len(),1);
        assert_eq!(products[0].title,"Test Product");
    }


}


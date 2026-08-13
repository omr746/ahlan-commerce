mod catalog;
use catalog::{Catalog,ProductCreate};
fn main() {
   let product=ProductCreate{
        title:"Laptop".to_string(),
        handle:"laptop".to_string(),
        price_cents:1000,
        inventory_quantity:10,
        published:true
    };
      let product2=ProductCreate{
        title:"iphone".to_string(),
        handle:"iphone".to_string(),
        price_cents:1000,
        inventory_quantity:10,
        published:true
    };
    let mut catalog=Catalog::new();
    catalog.create_product(product);
    catalog.create_product(product2); 
    let products=catalog.list_products();
    println!("All Products: {:?}",products);
    
}

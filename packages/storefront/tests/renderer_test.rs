//! Proves the "minimum HTML content" requirements from Task 13.1:
//! title, price, and inventory availability message must all appear.
//! Pure function, no Redis or Postgres needed -- nothing blocked here.

use catalog::Product;
use chrono::Utc;
use storefront::{ProductPageContext, render_product_page};
use uuid::Uuid;

fn sample_product(inventory_quantity: u32) -> Product {
    Product {
        id: Uuid::now_v7(),
        title: "Coffee Mug".to_string(),
        handle: "coffee-mug".to_string(),
        description: Some("Ceramic mug for daily coffee.".to_string()),
        price_cents: 2500,
        inventory_quantity,
        published: true,
        published_at: Some(Utc::now()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

#[test]
fn renders_title_price_and_availability() {
    let product = sample_product(12);
    let html = render_product_page(&ProductPageContext::from_product(&product));

    assert!(html.contains("Coffee Mug"), "product title must appear");
    assert!(html.contains("$25.00"), "product price must appear");
    assert!(
        html.contains("In stock (12 available)"),
        "availability must appear"
    );
}

#[test]
fn renders_out_of_stock_when_inventory_is_zero() {
    let product = sample_product(0);
    let html = render_product_page(&ProductPageContext::from_product(&product));
    assert!(html.contains("Out of stock"));
}

#[test]
fn renders_a_complete_html_document() {
    let product = sample_product(3);
    let html = render_product_page(&ProductPageContext::from_product(&product));

    assert!(html.starts_with("<!doctype html>"));
    assert!(html.contains("</html>"));
}

#[test]
fn escapes_user_supplied_title() {
    let mut product = sample_product(1);
    product.title = "<script>alert('xss')</script>".to_string();

    let html = render_product_page(&ProductPageContext::from_product(&product));

    assert!(
        !html.contains("<script>"),
        "raw script tag must not survive rendering"
    );
    assert!(html.contains("&lt;script&gt;"));
}

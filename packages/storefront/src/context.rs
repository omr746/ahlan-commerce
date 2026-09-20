//! Render context builder.
//!
//! This is the "what does the page need to know" layer. It does NOT load
//! anything (no DB, no cache) and it does NOT produce HTML. It takes a
//! `Product` that someone else already loaded and turns it into the exact
//! set of display-ready values the template consumes -- price formatting,
//! the availability sentence, etc.
//!
//! Keeping this separate from the renderer means the formatting decisions
//! (how cents become "$25.00", what "in stock" says) are unit-testable
//! without asserting against HTML strings.

use catalog::Product;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct ProductPageContext {
    pub product_id: Uuid,
    pub title: String,
    pub price_display: String,
    pub availability: String,
    pub description: Option<String>,
    pub product_updated_at: DateTime<Utc>,
}

impl ProductPageContext {
    pub fn from_product(product: &Product) -> Self {
        Self {
            product_id: product.id,
            title: product.title.clone(),
            price_display: format_price(product.price_cents),
            availability: availability_message(product.inventory_quantity),
            description: product.description.clone(),
            product_updated_at: product.updated_at,
        }
    }
}

fn format_price(price_cents: u32) -> String {
    let whole = price_cents / 100;
    let cents = price_cents % 100;
    format!("${whole}.{cents:02}")
}

/// The "inventory availability message" the minimum-HTML list requires.
fn availability_message(inventory_quantity: u32) -> String {
    if inventory_quantity == 0 {
        "Out of stock".to_string()
    } else {
        format!("In stock ({inventory_quantity} available)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_price_from_cents() {
        assert_eq!(format_price(2500), "$25.00");
        assert_eq!(format_price(2505), "$25.05");
        assert_eq!(format_price(0), "$0.00");
        assert_eq!(format_price(99), "$0.99");
    }

    #[test]
    fn reports_availability_from_inventory() {
        assert_eq!(availability_message(0), "Out of stock");
        assert_eq!(availability_message(1), "In stock (1 available)");
        assert_eq!(availability_message(12), "In stock (12 available)");
    }
}

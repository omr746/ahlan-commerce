//! Task 16.3 -- native-side tests.
//!
//! Rule enforced by this file's own content, not just its name: nothing
//! here references "external_id", "name", "slug", "price" (as a string),
//! "stock", or "is_visible" -- only native field names (title, handle,
//! price_cents, inventory_quantity, published). This proves native
//! product-create behavior is validated and tested on its own terms,
//! independent of whether a compatibility adapter exists at all. If the
//! adapter were deleted entirely, every test in this file would still
//! compile and still pass.

use api::dto::ProductCreateRequest;

fn valid_native_request() -> ProductCreateRequest {
    ProductCreateRequest {
        title: "Ceramic Travel Cup".to_string(),
        handle: "ceramic-travel-cup".to_string(),
        price_cents: 1899,
        inventory_quantity: 40,
        published: true,
        description: Some("Double-walled, keeps drinks hot for hours.".to_string()),
    }
}

#[test]
fn a_well_formed_native_request_passes_validation() {
    let request = valid_native_request();
    assert!(request.validate().is_ok());
}

#[test]
fn native_validation_rejects_an_empty_title() {
    let mut request = valid_native_request();
    request.title = "   ".to_string();
    assert!(request.validate().is_err());
}

#[test]
fn native_validation_rejects_an_empty_handle() {
    let mut request = valid_native_request();
    request.handle = "".to_string();
    assert!(request.validate().is_err());
}

#[test]
fn native_validation_rejects_an_invalid_handle_character_set() {
    let mut request = valid_native_request();
    request.handle = "Not A Valid Handle!".to_string();
    assert!(request.validate().is_err());
}

#[test]
fn native_validation_rejects_a_negative_price() {
    let mut request = valid_native_request();
    request.price_cents = -100;
    assert!(request.validate().is_err());
}

#[test]
fn native_validation_rejects_a_negative_inventory_quantity() {
    let mut request = valid_native_request();
    request.inventory_quantity = -1;
    assert!(request.validate().is_err());
}

#[test]
fn native_validation_accepts_an_unpublished_product_with_no_description() {
    let mut request = valid_native_request();
    request.published = false;
    request.description = None;
    assert!(request.validate().is_ok());
}
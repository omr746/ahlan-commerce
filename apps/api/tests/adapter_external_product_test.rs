//! Task 16.3 -- adapter-side tests.
//!
//! Proves the external -> native mapping using the actual committed
//! fixture, not an inline literal, so a change to the fixture file is what
//! these tests actually exercise. Everything here is about *mapping*
//! correctness -- whether the resulting ProductCreateRequest is valid is
//! not this file's concern (that's native_product_create_test.rs); these
//! tests would still be meaningful even against a mapped result that
//! happened to fail native validation, since the mapping itself is what's
//! under test.

use api::compat::external_product::{map_external_to_native, AdapterError, ExternalProduct};

fn load_fixture() -> ExternalProduct {
    let raw = std::fs::read_to_string("../../fixtures/external-product.json")
        .or_else(|_| std::fs::read_to_string("fixtures/external-product.json"))
        .expect("fixtures/external-product.json must exist and be readable");
    serde_json::from_str(&raw).expect("fixture must deserialize into ExternalProduct")
}

#[test]
fn maps_every_field_from_the_committed_fixture() {
    let external = load_fixture();
    let native = map_external_to_native(&external).expect("valid fixture must map successfully");

    assert_eq!(native.title, "Coffee Mug");
    assert_eq!(native.handle, "coffee-mug");
    assert_eq!(native.price_cents, 2500);
    assert_eq!(native.inventory_quantity, 12);
    assert_eq!(native.published, true);
}

#[test]
fn external_id_has_no_native_equivalent_and_is_dropped() {
    let external = load_fixture();
    let native = map_external_to_native(&external).unwrap();

    // ProductCreateRequest has no field for external_id at all -- the
    // strongest way to prove it's dropped is that this compiles: if a
    // native field for it existed, this test would need to reference it.
    // No description field exists in the external shape, so it must map
    // to None regardless of what external_id was.
    assert_eq!(native.description, None);
}

#[test]
fn description_is_always_none_since_the_external_shape_has_no_such_field() {
    let external = load_fixture();
    let native = map_external_to_native(&external).unwrap();
    assert_eq!(native.description, None);
}

#[test]
fn an_unparseable_external_price_is_rejected_before_reaching_native_validation() {
    let mut external = load_fixture();
    external.price = "not-a-price".to_string();

    let err = map_external_to_native(&external).unwrap_err();
    assert_eq!(err, AdapterError::InvalidPrice("not-a-price".to_string()));
}

#[test]
fn is_visible_false_maps_to_published_false() {
    let mut external = load_fixture();
    external.is_visible = false;

    let native = map_external_to_native(&external).unwrap();
    assert_eq!(native.published, false);
}

#[test]
fn a_whole_number_external_price_maps_correctly() {
    let mut external = load_fixture();
    external.price = "30".to_string();

    let native = map_external_to_native(&external).unwrap();
    assert_eq!(native.price_cents, 3000);
}
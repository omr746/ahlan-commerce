pub fn storefront_product_page_key(handle: &str) -> String {
    format!("storefront:product-page:{handle}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_the_expected_key_shape() {
        assert_eq!(
            storefront_product_page_key("coffee-mug"),
            "storefront:product-page:coffee-mug"
        );
    }
}

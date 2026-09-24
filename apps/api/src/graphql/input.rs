use async_graphql::InputObject;
use catalog::ProductCreate;
#[derive(InputObject)]
#[graphql(rename_fields = "camelCase")]
pub struct ProductCreateInput {
    pub title: String,
    pub handle: String,
    pub description: Option<String>,
    pub price_cents: i32,
    pub inventory_quantity: i32,
    pub published: bool,
}

impl ProductCreateInput {
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

        if self.inventory_quantity < 0 {
            return Err("inventory_quantity must be greater than or equal to 0.".to_string());
        }

        Ok(())
    }
}

impl From<ProductCreateInput> for ProductCreate {
    fn from(input: ProductCreateInput) -> Self {
        ProductCreate {
            title: input.title,
            handle: input.handle,
            price_cents: input.price_cents as u32,
            inventory_quantity: input.inventory_quantity as u32,
            published: input.published,
            description: input.description,
        }
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

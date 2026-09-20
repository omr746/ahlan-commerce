--! get_published_product_by_handle (handle) : (id, title, handle, description?, price_cents, inventory_quantity, published, published_at?, created_at, updated_at)

SELECT
    id,
    title,
    handle,
    description,
    price_cents,
    inventory_quantity,
    published,
    published_at,
    created_at,
    updated_at
FROM products
WHERE handle = :handle
  AND published = true;

--! update_product_publication (id, published, published_at?, updated_at) : (id, title, handle, description?, price_cents, inventory_quantity, published, published_at?, created_at, updated_at)

UPDATE products
SET
    published = :published,
    published_at = :published_at,
    updated_at = :updated_at
WHERE id = :id
RETURNING
    id,
    title,
    handle,
    description,
    price_cents,
    inventory_quantity,
    published,
    published_at,
    created_at,
    updated_at;


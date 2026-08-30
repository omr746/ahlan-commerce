--! create_product (id, title, handle, description?, price_cents, inventory_quantity, published, published_at?, created_at, updated_at) : (id, title, handle, description?, price_cents, inventory_quantity, published, published_at?, created_at, updated_at)
INSERT INTO products (
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
)
VALUES (
    :id,
    :title,
    :handle,
    :description,
    :price_cents,
    :inventory_quantity,
    :published,
    :published_at,
    :created_at,
    :updated_at
)
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
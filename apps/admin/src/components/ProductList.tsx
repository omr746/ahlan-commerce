import { useProductsQuery } from "../api/products";

function formatPrice(priceCents: number): string {
  return `$${(priceCents / 100).toFixed(2)}`;
}

export function ProductList() {
  const { data, isLoading, isError, error } = useProductsQuery();

  if (isLoading) {
    return <p data-testid="products-loading">Loading products…</p>;
  }

  if (isError) {
    return (
      <p data-testid="products-error" role="alert">
        Could not load products: {error.message}
      </p>
    );
  }

  if (!data || data.length === 0) {
    return <p data-testid="products-empty">No products yet. Create one above.</p>;
  }

  return (
    <table data-testid="products-table">
      <thead>
        <tr>
          <th>Title</th>
          <th>Handle</th>
          <th>Price</th>
          <th>Inventory</th>
          <th>Published</th>
          <th>CreatedAt</th>
          <th>UpdatedAt</th>
          <th>PublishedAt</th>
        </tr>
      </thead>
      <tbody>
        {data.map((product) => (
          <tr key={product.id} data-testid="product-row">
            <td>{product.title}</td>
            <td>{product.handle}</td>
            <td>{formatPrice(product.priceCents)}</td>
            <td>{product.inventoryQuantity}</td>
            <td>{product.published ? "Yes" : "No"}</td>
            <td>{new Date(product.createdAt).toLocaleString()}</td>
            <td>{new Date(product.updatedAt).toLocaleString()}</td>
            <td>{product.publishedAt ? new Date(product.publishedAt).toLocaleString() : "N/A"}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

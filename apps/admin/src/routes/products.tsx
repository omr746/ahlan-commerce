import { createRoute } from "@tanstack/react-router";
import { Route as rootRoute } from "./__root";
import { ProductList } from "../components/ProductList";
import { ProductForm } from "../components/ProductForm";

export const Route = createRoute({
  getParentRoute: () => rootRoute,
  path: "/products",
  component: ProductsPage,
});

function ProductsPage() {
  return (
    <section>
      <h2>Products</h2>
      <ProductForm />
      <ProductList />
    </section>
  );
}

import { createRoute, createRouter, Navigate } from "@tanstack/react-router";
import { Route as rootRoute } from "./routes/__root";
import { Route as productsRoute } from "./routes/products";

const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: () => <Navigate to="/products" />,
});

const routeTree = rootRoute.addChildren([indexRoute, productsRoute]);

export const router = createRouter({ routeTree });

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

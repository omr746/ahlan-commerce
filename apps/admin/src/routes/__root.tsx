import { createRootRoute, Outlet } from "@tanstack/react-router";

export const Route = createRootRoute({
  component: () => (
    <div className="app-shell">
      <header className="app-header">
        <h1>Ahlan Admin</h1>
      </header>
      <main>
        <Outlet />
      </main>
    </div>
  ),
});

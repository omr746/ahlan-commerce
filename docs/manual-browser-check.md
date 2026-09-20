# Manual Browser Check — Product Create/List Flow

## Why this doc exists

Task 10.4 asks for an automated e2e test for "create product" + "list update,"
and for this doc to be written **only if that automation is blocked**.

The e2e test itself is written and checked in at
`apps/admin/e2e/product-flow.spec.ts` (Playwright). It could not be *run* in
the sandbox this task was built in, because that sandbox's network egress
allowlist does not include Playwright's browser-binary CDN:

```
$ npx playwright install chromium
Error: Download failed: server returned code 403
body 'Host not in allowlist: cdn.playwright.dev. Add this host to your
network egress settings to allow access.'
```

Everything else in the flow was verified without a real browser:

- The GraphQL API (`server/`) starts and answers `products` and
  `createProduct` over HTTP.
- The admin dev server (`apps/admin`) builds, typechecks, and serves
  `/products`.
- A scripted HTTP client exercised the same GraphQL calls the UI makes
  (`createProduct` then `products`) and confirmed the new product appears
  in the list response.

What was **not** verified in this environment is the actual browser
rendering and click-through — that's what the manual steps below cover,
and what the checked-in Playwright test will cover automatically once run
in an environment with normal network access (a mentor's machine or CI).

## How to unblock automation

On a machine or CI runner with normal internet access:

```bash
cd apps/admin
npm install
npx playwright install chromium
npm run test:e2e
```

`npm run test:e2e` starts the Vite dev server itself (see
`playwright.config.ts`) but expects the GraphQL API from `server/` to
already be running on `http://localhost:4000` — start that first:


## Manual browser check (mentor-repeatable)

If you want to eyeball the flow without running Playwright at all:

1. **Start the API**
   ```bash
   cd server
   npm install
   npm run dev
   ```
   Confirm it logs `GraphQL API ready at http://localhost:3000/graphql`.

2. **Start the admin app**
   ```bash
   cd apps/admin
   npm install
   npm run dev
   ```
   Confirm it logs a local URL, typically `http://localhost:5173`.

3. **Open the app**
   Visit `http://localhost:5173/products` in a browser. You should see:
   - An "Ahlan Admin" header and a "Products" heading.
   - A create-product form (title, handle, description, price in cents,
     inventory quantity, published checkbox).
   - A table listing the one seeded product, "Classic Tote Bag."

4. **Create a product**
   Fill in the form (e.g. title "Manual Check Product", handle
   `manual-check-product`, price cents `1500`) and click **Create
   product**.

5. **Confirm the list updates without a reload**
   - A "Product created." message appears under the form.
   - The new row appears in the table immediately, with no visible page
     refresh (watch the browser tab's loading spinner — it should not
     spin for a full navigation).
   - Open browser dev tools → Network tab beforehand if you want to
     confirm this concretely: you should see one `POST /graphql` for the
     mutation and one more `POST /graphql` for the refetch, with no
     document (HTML page) request in between.

6. **Confirm persistence across a real reload**
   Refresh the page. The manually created product should still be in the
   list, confirming it was written to the API/store and not just added to
   local UI state.

## Approval

This manual procedure, plus the checked-in but not-yet-run Playwright
test, is the deliverable for Task 10.4 given the sandbox's network
restriction. A mentor should either:

- run `npm run test:e2e` in an unrestricted environment and confirm it
  passes, or
- walk through the manual steps above and sign off here.

import { test, expect } from "@playwright/test";

test("creating a product updates the list without a full page refresh", async ({
  page,
}) => {
  await page.goto("/products");

  await expect(page.getByRole("heading", { name: "Products" })).toBeVisible();

  const uniqueHandle = `e2e-test-product-${Date.now()}`;

  await page.getByLabel("Title").fill("E2E Test Product");
  await page.getByLabel("Handle").fill(uniqueHandle);
  await page.getByLabel("Price (cents)").fill("1999");
  await page.getByLabel("Inventory quantity").fill("10");

  await page.getByRole("button", { name: "Create product" }).click();

  await expect(page.getByTestId("create-success")).toBeVisible();

  const row = page.getByTestId("product-row").filter({ hasText: uniqueHandle });
  await expect(row).toBeVisible();
  await expect(row).toContainText("E2E Test Product");
  await expect(row).toContainText("$19.99");
});

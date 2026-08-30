# Product Scenario Specs

Scenarios for the behaviors described in `specs/product-prd.md`. Each one
follows the same structure so any scenario can be reviewed the same way,
regardless of who wrote it or when.

---

## PRD-PROD-001 - Valid Product Create

Version: 1 - 2026-08-29

Intent:
Protects a seller's ability to add a product with correct input and get
back a fully-formed record - a generated id, both timestamps, and a
publish timestamp that's present exactly when the product was created
published.

Given:
- No product exists yet with the handle this scenario will use.
- A request body with a title, a unique lowercase/hyphen handle, a
  non-negative price, a non-negative inventory count, and `published: true`.

When:
- A create-product request is made with that body.

Then:
- Response status is `201 Created`.
- The response body includes a generated `id` not supplied by the caller.
- `handle` in the response matches the request exactly.
- `published` is `true` and `published_at` is present (not null).
- `created_at` and `updated_at` are both present.

Verification:
Automated by: prd_prod_001_valid_product_create

Review:
Status: Pending
Reviewed version: none
Reviewed by: none
Reviewed at: none

---

## PRD-PROD-002 - Duplicate Handle Rejected

Version: 1 - 2026-08-29

Intent:
Protects catalog integrity: two products can never share a handle, and a
rejected duplicate attempt must never mutate the product that already
holds that handle.

Given:
- A product already exists with a specific handle, title, price, and
  inventory count.

When:
- A second create-product request is made using that exact same handle,
  with different title/price/inventory values.

Then:
- Response status is `409 Conflict`.
- Response body's `error.code` is `duplicate_product_handle`.
- No second product is created.
- The original product's title, price, and inventory count are unchanged
  when re-fetched afterward.

Verification:
Automated by: prd_prod_002_duplicate_handle_rejected

Review:
Status: Pending
Reviewed version: none
Reviewed by: none
Reviewed at: none

---

## PRD-PROD-003 - List Empty Products

Version: 1 - 2026-08-29

Intent:
Protects the guarantee that an empty result is a normal, successful
answer - a seller with no matching products yet must never see an error
where they should see an empty list.

Given:
- A specific product handle that has never been created.

When:
- A request is made to list published products.

Then:
- Response status is `200 OK`.
- Response body is a list (possibly containing other products from
  elsewhere in the system, but never an error or `null`).
- The never-created handle from Given does not appear anywhere in the list.

Verification:
Automated by: prd_prod_003_list_empty_products

Review:
Status: Pending
Reviewed version: none
Reviewed by: none
Reviewed at: none

---

## PRD-PROD-004 - List Persisted Products

Version: 1 - 2026-08-29

Intent:
Protects the two list views' filtering contract: the all-products list
must include everything regardless of publish state, and the
published-only list must genuinely exclude anything not published, not
merely leave it unmarked.

Given:
- One published product and one unpublished (draft) product have both
  been created, each with a distinct handle.

When:
- A request is made to list all products, and separately, a request is
  made to list published-only products.

Then:
- The all-products response is `200 OK` and includes both the published
  and the draft product's handles.
- The published-only response is `200 OK`, includes the published
  product's handle, and does not include the draft product's handle
  anywhere in the response.

Verification:
Automated by: prd_prod_004_list_persisted_products

Review:
Status: Pending
Reviewed version: none
Reviewed by: none
Reviewed at: none

---

## PRD-PROD-005 - Invalid Create Input Rejected

Version: 1 - 2026-08-29

Intent:
Protects data quality at the point of entry: no product is ever created
from a request that violates title, handle, price, or inventory rules,
and the caller is always told clearly that the problem was their input,
not a system failure.

Given:
- A create-product request body violating exactly one rule at a time:
  blank title, malformed (non-lowercase/non-hyphen) handle, negative
  price, or negative inventory count.

When:
- A create-product request is made with each such body in turn.

Then:
- For every case, response status is `400 Bad Request`.
- For every case, response body's `error.code` is `validation_failed`.
- For every case, no product is created.

Verification:
Automated by: prd_prod_005_invalid_create_input_rejected

Review:
Status: Pending
Reviewed version: none
Reviewed by: none
Reviewed at: none
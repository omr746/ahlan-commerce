# Product PRD: Create and List Products

## Overview

Ahlan Commerce needs a way for a seller to add products to their catalog
and see what's in it. This document describes what "create a product"
and "list products" mean from the outside - what a caller can rely on,
regardless of how it's implemented or which system it's implemented in.
Nothing in this document requires reading source code to verify; every
claim below can be checked by making a request and looking at the
response.

## Problem

A seller cannot sell anything until their products exist somewhere the
storefront, search, and order systems can all agree on. Before this
exists, there is no single source of truth for "what products does this
seller have, and what do we know about each one."

## Goals

- A seller (or a system acting for them) can add a new product with a
  title, a unique handle, a price, a stock count, and whether it's
  published, with an optional description.
- Anyone with access can retrieve the full list of a seller's products,
  and separately, just the ones that are published.
- The rules for what makes a product valid are enforced consistently and
  explained clearly when they're violated - no product is ever created
  half-valid, and no caller is ever left guessing why a request failed.

## Non-goals

- Editing a product's title, handle, price, or inventory count after
  creation. (Updating whether a product is published, and its
  description, exists separately - it is not part of this PRD.)
- Deleting products.
- Anything about carts, orders, checkout, or payment - this is only
  about the catalog existing.
- Access control (who is allowed to create or view products) - every
  request in this PRD is assumed to already be authorized.

## User Stories

**As a seller, I want to add a product to my catalog, so that it can
eventually be shown to customers.**

I provide a title, a handle (the URL-friendly identifier customers and
search engines will see), a price, how many I have in stock, and whether
it should be visible yet. I optionally provide a description. When I do
this successfully, I'm told the product now exists, and I'm given back
everything about it, including a unique identifier and timestamps I
didn't have to supply myself.

**As a seller (or a system displaying my catalog), I want to see every
product I've created, so that I can review or display my full catalog.**

I ask for my product list and get back every product I've created,
regardless of whether it's published. Whether I have zero products or
many, I get a clear, predictable answer - never an error just because
the catalog happens to be empty.

## Functional Requirements

### Creating a product

A product is created from:

| Field | Required? | Rule |
|---|---|---|
| `title` | Yes | Must not be blank (empty or whitespace-only is rejected). |
| `handle` | Yes | Must not be blank, must be unique across all products, and must be lowercase letters, numbers, and hyphens only. |
| `description` | No | If omitted or explicitly empty, the product simply has no description. |
| `price_cents` | Yes | Whole number, must be zero or greater. |
| `inventory_quantity` | Yes | Whole number, must be zero or greater. |
| `published` | Yes | `true` or `false`. |

On success, the caller receives back the product exactly as stored,
including:

- A unique identifier, generated for this product and never reused.
- The publish timestamp: present if the product was created as
  published, absent if it was created unpublished.
- A created timestamp and an updated timestamp, both set to the moment
  of creation.

Creating a product never requires the caller to supply an identifier or
any timestamp themselves.

### Listing products

Two things a caller can ask for:

- **All products.** Every product that exists, regardless of published
  state, ordered oldest-created first.
- **Published products only.** The same list, filtered to only products
  currently published, ordered most-recently-published first.

Both are always a list - possibly empty, never an error on their own
account of being empty.

## Acceptance Criteria

### AC1 - Valid create succeeds

**Given** a request to create a product with a title, a unique and
correctly-formatted handle, a non-negative price, a non-negative
inventory count, and a published flag,
**when** the request is made,
**then** the product is created, the caller receives it back with a
newly-generated unique identifier, a created timestamp and updated
timestamp both set to now, and - if `published` was `true` - a publish
timestamp also set to now; if `published` was `false`, no publish
timestamp is present.

### AC2 - Duplicate handle is rejected

**Given** a product already exists with a given handle,
**when** a request is made to create another product using that exact
same handle,
**then** no new product is created, the existing product is left
completely unchanged, and the caller receives a clear, specific error
telling them the handle is already in use - not a generic failure, and
not a silent success that quietly overwrites or duplicates anything.

### AC3 - Listing with no products returns an empty list

**Given** no products exist yet,
**when** a request is made to list products,
**then** the caller receives a successful response containing an empty
list - never an error, and never treated as "not found."

### AC4 - Listing with products returns all of them

**Given** one or more products exist, some published and some not,
**when** a request is made to list all products,
**then** the caller receives every one of them, published or not, in the
order they were created (oldest first).

**Given** the same products,
**when** a request is made to list only published products,
**then** the caller receives only the ones currently published, ordered
most-recently-published first, and every unpublished product is absent
from that list - not merely unmarked, genuinely absent.

### AC5 - Invalid input is rejected clearly

**Given** a request to create a product that violates one or more rules
from the Functional Requirements above (blank title, blank or malformed
handle, negative price, or negative inventory count),
**when** the request is made,
**then** no product is created, and the caller receives a clear error
identifying that the request was invalid and, where practical, which
rule was violated - never a request that partially succeeds, and never a
failure that looks the same as a duplicate handle or a system outage.

## What "done" looks like

Every acceptance criterion above can be verified by a person with no
access to the source code: make the request described, and check that
the described outcome is what actually happens. If any of AC1-AC5 can
only be confirmed by reading implementation code rather than by making a
request and observing the response, this PRD has not been met.

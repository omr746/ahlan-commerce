//! Task 14.1 -- writes both generated documentation artifacts.
//!
//!   cargo run -p api --bin export-docs
//!
//! Needs no database, no Redis, and no running server: the OpenAPI spec
//! comes from the route/handler types and the GraphQL SDL from the schema
//! types. That's what makes this safe to run in CI for `docs-api-check`.
//!
//! Output is written to `docs/generated/`, resolved relative to the
//! current working directory, so run this from the repo root (the Makefile
//! target does).

use std::fs;
use std::path::Path;

use api::graphql::schema::schema_sdl;
use api::openapi::openapi_spec;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = Path::new("docs/generated");
    fs::create_dir_all(out_dir)?;

    // --- REST: docs/generated/openapi.json
    //
    // `to_pretty_json` (not `to_json`) on purpose: the file is checked in
    // and diffed by `docs-api-check`, so a stable multi-line format makes
    // review diffs readable instead of one enormous changed line.
    let openapi_path = out_dir.join("openapi.json");
    let openapi_json = openapi_spec().to_pretty_json()?;
    fs::write(&openapi_path, format!("{openapi_json}\n"))?;
    println!("wrote {}", openapi_path.display());

    // --- GraphQL: docs/generated/schema.graphql
    let schema_path = out_dir.join("schema.graphql");
    let sdl = schema_sdl();
    fs::write(&schema_path, &sdl)?;
    println!("wrote {}", schema_path.display());

    Ok(())
}

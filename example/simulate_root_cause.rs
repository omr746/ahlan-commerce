
use rootcause::prelude::*;
use uuid::Uuid;
mod error;
use crate::error::AppError;

/// Stands in for a real dependency call (Postgres, Redis, ...) that
/// fails. `rootcause::ResultExt::context` wraps the raw `io::Error` in a
/// `Report`, adding a human sentence describing what we were doing when
/// it failed - that sentence, the io error's message, and the eventual
/// `.context()` added by the caller all end up in the tree `log()` prints.
fn simulate_cache_dependency_call() -> Result<(), Report> {
    let raw: std::io::Result<()> = Err(std::io::Error::new(
        std::io::ErrorKind::ConnectionRefused,
        "connection refused: internal-cache:6379",
    ));
    raw.context("checking product availability against the cache layer")?;
    Ok(())
}

fn main() {
    let request_id = Uuid::now_v7();

    let cause = simulate_cache_dependency_call()
        .context("create_product failed because a dependency was unavailable")
        .unwrap_err();

    let err = AppError::dependency_unavailable(request_id, cause.into());

    // Engineer-facing: full cause, printed to stderr only.
    err.log();

    // Client-facing: safe envelope, printed to stdout only.
    let body = err.body();
    println!("{}", serde_json::to_string_pretty(&body).unwrap());
}

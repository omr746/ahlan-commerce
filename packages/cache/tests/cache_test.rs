//! Requires a reachable Redis (default redis://127.0.0.1:6379, override
//! with REDIS_URL) — run `make redis-health` first. The fallback tests
//! point at an unreachable address on purpose, so they need no Redis.

use std::time::Duration;

use cache::Cache;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Sample {
    value: String,
}

fn redis_url() -> String {
    std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string())
}

const UNREACHABLE_REDIS_URL: &str = "redis://127.0.0.1:1";

fn unique_key(prefix: &str) -> String {
    format!("cache-test:{prefix}:{}", uuid_like())
}

fn uuid_like() -> String {
    format!(
        "{:x}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    )
}

#[tokio::test]
async fn set_then_get_returns_the_stored_value() {
    let cache = Cache::new(&redis_url()).expect("valid redis url");
    let key = unique_key("set-get");

    cache
        .set_json(
            &key,
            &Sample {
                value: "hello".into(),
            },
            Duration::from_secs(30),
        )
        .await;

    let got: Option<Sample> = cache.get_json(&key).await;
    assert_eq!(
        got,
        Some(Sample {
            value: "hello".into()
        })
    );

    cache.delete(&key).await;
}

#[tokio::test]
async fn get_on_a_missing_key_returns_none() {
    let cache = Cache::new(&redis_url()).expect("valid redis url");
    let key = unique_key("missing");
    let got: Option<Sample> = cache.get_json(&key).await;
    assert_eq!(got, None);
}

#[tokio::test]
async fn delete_removes_the_key() {
    let cache = Cache::new(&redis_url()).expect("valid redis url");
    let key = unique_key("delete");

    cache
        .set_json(
            &key,
            &Sample {
                value: "to-remove".into(),
            },
            Duration::from_secs(30),
        )
        .await;
    assert_eq!(
        cache.get_json::<Sample>(&key).await,
        Some(Sample {
            value: "to-remove".into()
        })
    );

    cache.delete(&key).await;

    assert_eq!(cache.get_json::<Sample>(&key).await, None);
}

#[tokio::test]
async fn set_respects_ttl_expiry() {
    let cache = Cache::new(&redis_url()).expect("valid redis url");
    let key = unique_key("ttl");

    cache
        .set_json(
            &key,
            &Sample {
                value: "short-lived".into(),
            },
            Duration::from_secs(1),
        )
        .await;
    assert_eq!(
        cache.get_json::<Sample>(&key).await,
        Some(Sample {
            value: "short-lived".into()
        })
    );

    tokio::time::sleep(Duration::from_millis(1200)).await;

    assert_eq!(cache.get_json::<Sample>(&key).await, None);
}

#[tokio::test]
async fn get_returns_none_when_redis_is_unreachable_rather_than_erroring() {
    let cache = Cache::new(UNREACHABLE_REDIS_URL).expect("valid redis url");
    let got: Option<Sample> = cache.get_json("anything").await;
    assert_eq!(
        got, None,
        "an unreachable Redis must look like a cache miss, not a crash"
    );
}

#[tokio::test]
async fn set_does_not_panic_or_block_forever_when_redis_is_unreachable() {
    let cache = Cache::new(UNREACHABLE_REDIS_URL).expect("valid redis url");
    cache
        .set_json(
            "anything",
            &Sample { value: "x".into() },
            Duration::from_secs(30),
        )
        .await;
}

#[tokio::test]
async fn delete_does_not_panic_when_redis_is_unreachable() {
    let cache = Cache::new(UNREACHABLE_REDIS_URL).expect("valid redis url");
    cache.delete("anything").await;
}

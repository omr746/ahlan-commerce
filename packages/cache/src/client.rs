use std::time::Duration;

use redis::AsyncCommands;
use serde::Serialize;
use serde::de::DeserializeOwned;

#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    /// Not a runtime-availability problem — the configured REDIS_URL
    /// itself is malformed. Worth failing startup over.
    #[error("invalid redis url: {0}")]
    InvalidUrl(#[source] redis::RedisError),
}

#[derive(Clone)]
pub struct Cache {
    client: redis::Client,
}

impl Cache {
    /// `redis::Client::open` only parses the URL — it does not connect.
    /// A Redis outage at process startup should never prevent `api` or
    /// `import-worker` from starting.
    pub fn new(redis_url: &str) -> Result<Self, CacheError> {
        let client = redis::Client::open(redis_url).map_err(CacheError::InvalidUrl)?;
        Ok(Self { client })
    }

    /// `Some(value)` on a hit, `None` on a miss OR any Redis/deserialize
    /// failure — callers can't (and shouldn't) tell those apart.
    pub async fn get_json<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        let mut conn = match self.client.get_multiplexed_async_connection().await {
            Ok(conn) => conn,
            Err(err) => {
                tracing::warn!(
                    cache_key = %key,
                    error = %redact(&err),
                    "cache unavailable, falling back to source"
                );
                return None;
            }
        };

        let raw: Option<String> = match conn.get(key).await {
            Ok(v) => v,
            Err(err) => {
                tracing::warn!(
                    cache_key = %key,
                    error = %redact(&err),
                    "cache read failed, falling back to source"
                );
                return None;
            }
        };

        match raw {
            None => {
                tracing::info!(cache_key = %key, cache_event = "miss", "cache miss");
                None
            }
            Some(payload) => match serde_json::from_str::<T>(&payload) {
                Ok(value) => {
                    tracing::info!(cache_key = %key, cache_event = "hit", "cache hit");
                    Some(value)
                }
                Err(err) => {
                    tracing::warn!(
                        cache_key = %key,
                        error = %err,
                        "cached value failed to deserialize, treating as miss"
                    );
                    None
                }
            },
        }
    }

    /// Best-effort write. Never fails the caller.
    pub async fn set_json<T: Serialize>(&self, key: &str, value: &T, ttl: Duration) {
        let mut conn = match self.client.get_multiplexed_async_connection().await {
            Ok(conn) => conn,
            Err(err) => {
                tracing::warn!(
                    cache_key = %key,
                    error = %redact(&err),
                    "cache unavailable, skipping write"
                );
                return;
            }
        };

        let payload = match serde_json::to_string(value) {
            Ok(p) => p,
            Err(err) => {
                tracing::error!(cache_key = %key, error = %err, "failed to serialize value for cache write");
                return;
            }
        };

        let ttl_secs = ttl.as_secs().max(1);
        let result: redis::RedisResult<()> = conn.set_ex(key, payload, ttl_secs).await;
        if let Err(err) = result {
            tracing::warn!(
                cache_key = %key,
                error = %redact(&err),
                "cache write failed, continuing without cache"
            );
        }
    }

    /// Best-effort delete. Same fallback behavior as set_json.
    pub async fn delete(&self, key: &str) {
        let mut conn = match self.client.get_multiplexed_async_connection().await {
            Ok(conn) => conn,
            Err(err) => {
                tracing::warn!(
                    cache_key = %key,
                    error = %redact(&err),
                    "cache unavailable, skipping delete"
                );
                return;
            }
        };

        let result: redis::RedisResult<()> = conn.del(key).await;
        if let Err(err) = result {
            tracing::warn!(
                cache_key = %key,
                error = %redact(&err),
                "cache delete failed, continuing without cache"
            );
        }
    }
}

/// Defensive redaction: strip anything that looks like `scheme://...@`
/// down to `scheme://<redacted>@` before it ever reaches a log line, in
/// case a server-reported error ever echoes a URL with credentials back.
fn redact(err: &redis::RedisError) -> String {
    let msg = err.to_string();
    let Some(scheme_end) = msg.find("://") else {
        return msg;
    };
    let Some(at_offset) = msg[scheme_end..].find('@') else {
        return msg;
    };
    let at_index = scheme_end + at_offset;
    format!("{}://<redacted>{}", &msg[..scheme_end], &msg[at_index..])
}

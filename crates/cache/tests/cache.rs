use std::time::Duration;

use cds_cache::Cache;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Payload {
    value: String,
}

// Set CDSCTF_TEST_CACHE_URL to run this against a real Redis/Valkey instance.
// The test remains a no-op in environments where no cache service is present.
#[tokio::test]
async fn exercises_atomic_and_typed_operations() {
    let Some(url) = std::env::var_os("CDSCTF_TEST_CACHE_URL") else {
        return;
    };
    let prefix = format!("cds-cache-test-{}", std::process::id());
    let cache = Cache::connect(
        url.to_str().expect("test URL must be UTF-8"),
        &prefix,
        Duration::from_secs(2),
        Duration::from_secs(2),
        16,
    )
    .await
    .expect("connect to test Valkey");

    let payload = Payload {
        value: "legacy-json-compatible".to_owned(),
    };
    cache.set("value", &payload).await.expect("set JSON value");
    assert_eq!(cache.get::<Payload>("value").await.unwrap(), Some(payload));

    let legacy = br#"{"value":"written-by-fred"}"#;
    let _: () = cache
        .query(
            cds_cache::redis::cmd("SET")
                .arg(cache.key("legacy"))
                .arg(legacy),
        )
        .await
        .expect("write legacy JSON bytes");
    assert_eq!(
        cache.get::<Payload>("legacy").await.unwrap(),
        Some(Payload {
            value: "written-by-fred".to_owned()
        })
    );

    assert!(
        cache
            .set_if_absent("once", &1_u8, Duration::from_secs(30))
            .await
            .unwrap()
    );
    assert!(
        !cache
            .set_if_absent("once", &2_u8, Duration::from_secs(30))
            .await
            .unwrap()
    );
    assert_eq!(cache.take::<u8>("once").await.unwrap(), Some(1));
    assert_eq!(cache.take::<u8>("once").await.unwrap(), None);

    let first = cache
        .fixed_window("limit", 2, Duration::from_secs(30))
        .await
        .unwrap();
    let second = cache
        .fixed_window("limit", 2, Duration::from_secs(30))
        .await
        .unwrap();
    let third = cache
        .fixed_window("limit", 2, Duration::from_secs(30))
        .await
        .unwrap();
    assert!(first.allowed && second.allowed && !third.allowed);
    assert_eq!(third.remaining, 0);

    assert!(cache.clear_namespace().await.unwrap() >= 3);
    assert!(!cache.exists("value").await.unwrap());
}

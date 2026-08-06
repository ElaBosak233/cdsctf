# fred to redis-rs / Valkey migration

## Decision

`cds-cache` now uses `redis-rs 1.5` and its Tokio `ConnectionManager`. The
manager is cloneable, multiplexed, reconnects with exponential backoff, and
applies connection, response, and in-flight request limits from `env.cache`.
Valkey speaks the Redis wire protocol, and the payload encoding remains
compatible. The new namespace intentionally leaves old transient keys behind
to expire instead of copying them.

The typed cache format remains JSON. This is deliberate: it preserves existing
values, makes operational inspection straightforward, and avoids coupling the
application cache to a module such as RedisJSON. Session records remain
MessagePack, but now live below `cdsctf:session:` by default.

## What changes

| Area | Before | Now |
| --- | --- | --- |
| Client | fred 10.1 | redis-rs 1.5 ConnectionManager |
| Key ownership | unscoped keys | configurable `cdsctf:` namespace |
| TTL | seconds-only helper | `Duration`, millisecond precision |
| One-time values | GETDEL | `take` (GETDEL) |
| Rate limits | GET + INCR/SETEX race | one-key Lua fixed-window decision |
| Cleanup | FLUSHALL | namespace SCAN + UNLINK |
| Advanced Valkey | client API leaked | `client`, `connection`, `query`, and `redis` re-export |

## Compatibility and rollout

1. Use a coordinated cutover rather than mixing old and new binaries. Existing
   unscoped, short-lived application keys will naturally age out; old session
   cookies are invalidated because sessions are deliberately isolated under
   the new namespace.
2. Do not point this deployment at a database shared with unrelated services
   unless `CDSCTF_CACHE__KEY_PREFIX` is unique.
3. Set `CDSCTF_TEST_CACHE_URL` and run `cargo test -p cds-cache` against a
   disposable Valkey instance for release verification.

The wrapper never issues `FLUSHALL`. `clear_namespace` is incremental and only
matches its configured prefix, but it is still intended for tests or explicit
administrative actions.

The submission limiter now admits exactly 10 attempts per 60-second window.
The old `limit > 10` check unintentionally admitted 11.

## Valkey feature boundary

The dependency enables ACL, Streams, scripting, geospatial, Bloom, JSON,
vector-set, TLS, Cluster, and Sentinel command support. The typed cache keeps a
small stable surface; use `Cache::client()` for dedicated Pub/Sub or blocking
stream connections and `Cache::query()`/`Cache::connection()` for commands not
yet wrapped. Build Cluster and Sentinel clients with the re-exported `redis`
crate; they require topology-specific configuration and should not be guessed
from a single-node URL. Cluster deployments must keep all keys passed to one
command in a single hash slot, as required by Valkey.

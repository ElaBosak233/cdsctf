//! Valkey-backed CQRS read-model score engine.
//!
//! # Consistency without database write locks
//!
//! - PostgreSQL holds only immutable ground-truth facts (`submissions`,
//!   `game_challenges`, `teams.state`). Derived scores and ranks are **never**
//!   written back to those tables.
//! - Shared score state lives solely in Valkey — there is no per-pod L1 cache
//!   or background invalidation listener to keep coherent.
//! - After any score-affecting PostgreSQL commit, writers call
//!   [`Score::invalidate`], which increments a monotonic Valkey epoch
//!   (`INCR score:ver:{game_id}`).
//! - Rebuilds always read `captured_ver = GET score:ver:{game_id}` **before**
//!   querying PostgreSQL, then commit the MessagePack snapshot via an atomic
//!   Lua CAS script (`my_ver >= snap_ver`). Therefore:
//!   1. A slower older rebuild can never overwrite a newer snapshot.
//!   2. Any DB commit concurrent with a rebuild increments `ver` *after*
//!      `captured_ver`, guaranteeing the next reader sees `snap_ver < cur_ver`
//!      and triggers a follow-up rebuild.
//! - A short Valkey lock (`SET NX PX`) provides single-flight stampede
//!   protection; concurrent readers serve the previous snapshot while a peer
//!   rebuilds. If Valkey is temporarily unavailable, reads compute directly
//!   from PostgreSQL without failing.

pub mod math;
pub mod snapshot;

use std::time::{Duration, Instant};

use cds_cache::Cache;
use cds_db::{DB, game_challenge, submission, team};
use redis::AsyncCommands;
use tracing::{debug, warn};

pub use crate::snapshot::{ChallengeScore, GameScoreSnapshot, RankedTeam, SubmissionScore};

/// Valkey TTL for cached score snapshots (recomputed automatically on miss).
const SNAPSHOT_TTL_SECS: u64 = 600;

/// Cluster-wide single-flight rebuild lock TTL.
const REBUILD_LOCK_TTL_MS: u64 = 3_000;

/// Atomic Valkey Lua CAS: stores the MessagePack snapshot only if its captured
/// epoch is at least as new as the currently stored snapshot epoch.
const STORE_SNAPSHOT_SCRIPT: &str = r#"
local snap_ver = tonumber(redis.call('HGET', KEYS[1], 'ver') or '-1')
local my_ver = tonumber(ARGV[1])
if my_ver >= snap_ver then
    redis.call('HSET', KEYS[1], 'ver', ARGV[1], 'data', ARGV[2])
    redis.call('EXPIRE', KEYS[1], tonumber(ARGV[3]))
    return 1
end
return 0
"#;

/// Atomic compare-and-delete for releasing the cluster single-flight lock.
const RELEASE_LOCK_SCRIPT: &str = r#"
if redis.call('GET', KEYS[1]) == ARGV[1] then
    return redis.call('DEL', KEYS[1])
end
return 0
"#;

/// Stateless, cloneable handle to the CQRS score read-model shared by Axum
/// handlers and the checker worker.
#[derive(Clone)]
pub struct Score {
    db: DB,
    cache: Cache,
}

impl Score {
    /// Creates a new [`Score`] backed by PostgreSQL and Valkey.
    pub fn new(db: DB, cache: Cache) -> Self {
        Self {
            db,
            cache: cache.scoped("score"),
        }
    }

    fn ver_key(&self, game_id: i64) -> String {
        self.cache.key(format!("ver:{game_id}"))
    }

    fn snap_key(&self, game_id: i64) -> String {
        self.cache.key(format!("snap:{game_id}"))
    }

    fn lock_key(&self, game_id: i64) -> String {
        self.cache.key(format!("lock:{game_id}"))
    }

    /// Marks `game_id`'s cached score projection stale after a committed
    /// PostgreSQL write by advancing its monotonic Valkey epoch.
    ///
    /// Never writes to PostgreSQL and never fails the caller.
    pub async fn invalidate(&self, game_id: i64) {
        let mut conn = self.cache.connection();
        match conn.incr::<_, _, u64>(self.ver_key(game_id), 1_u64).await {
            Ok(ver) => {
                debug!(game_id, version = ver, "score epoch incremented");
            }
            Err(error) => {
                warn!(game_id, ?error, "score epoch increment failed");
            }
        }
    }

    /// Advances `game_id`'s epoch and immediately rebuilds and publishes a
    /// fresh snapshot to Valkey.
    pub async fn force_refresh(&self, game_id: i64) -> Result<GameScoreSnapshot, anyhow::Error> {
        self.invalidate(game_id).await;
        self.rebuild_and_store(game_id).await
    }

    /// Returns the latest score snapshot for `game_id`, serving from Valkey
    /// when fresh and transparently recomputing on miss or epoch advance.
    pub async fn snapshot(&self, game_id: i64) -> Result<GameScoreSnapshot, anyhow::Error> {
        // 1. Read current Valkey epoch; fall back directly to PostgreSQL if
        //    Valkey is temporarily unreachable.
        let current_ver = match self.read_epoch(game_id).await {
            Ok(ver) => ver,
            Err(error) => {
                warn!(
                    game_id,
                    ?error,
                    "valkey epoch lookup failed; computing snapshot directly from PostgreSQL"
                );
                return self.compute_from_db(game_id, 0).await;
            }
        };

        // 2. Serve from Valkey if the cached snapshot covers `current_ver`.
        let cached = match self.read_snapshot(game_id).await {
            Ok(snap) => snap,
            Err(error) => {
                warn!(game_id, ?error, "valkey snapshot read failed");
                None
            }
        };
        if let Some(ref snap) = cached
            && snap.version >= current_ver
        {
            return Ok(snap.clone());
        }

        // 3. Acquire the single-flight lock so only one caller rebuilds.
        let token = nanoid::nanoid!();
        let acquired = self.try_acquire_lock(game_id, &token).await.unwrap_or(true);
        if !acquired {
            // Another worker/pod is actively rebuilding. Serve the previous
            // snapshot immediately if present; on a completely cold cache wait
            // briefly for the peer to finish before computing locally.
            if let Some(stale) = cached {
                return Ok(stale);
            }
            tokio::time::sleep(Duration::from_millis(25)).await;
            if let Ok(Some(fresh)) = self.read_snapshot(game_id).await {
                return Ok(fresh);
            }
            return self.compute_from_db(game_id, current_ver).await;
        }

        let result = self.rebuild_and_store(game_id).await;
        self.release_lock(game_id, &token).await;
        result
    }

    /// Reads `captured_ver` **before** querying PostgreSQL, computes the
    /// snapshot, and writes it to Valkey via Lua CAS.
    async fn rebuild_and_store(&self, game_id: i64) -> Result<GameScoreSnapshot, anyhow::Error> {
        let started = Instant::now();
        let captured_ver = self.read_epoch(game_id).await.unwrap_or(0);
        let snap = self.compute_from_db(game_id, captured_ver).await?;

        if let Err(error) = self.write_snapshot_cas(game_id, &snap).await {
            warn!(game_id, ?error, "valkey snapshot CAS write failed");
        }

        debug!(
            game_id,
            version = captured_ver,
            teams = snap.ranked_teams.len(),
            solves = snap.submission_scores.len(),
            elapsed_ms = started.elapsed().as_millis(),
            "score snapshot built"
        );
        Ok(snap)
    }

    async fn compute_from_db(
        &self,
        game_id: i64,
        version: u64,
    ) -> Result<GameScoreSnapshot, anyhow::Error> {
        let (submissions, challenges, passed_team_ids) = tokio::try_join!(
            submission::find_score_inputs(&self.db.conn, game_id),
            game_challenge::find_score_inputs(&self.db.conn, game_id),
            team::find_passed_ids(&self.db.conn, game_id),
        )?;

        Ok(snapshot::build(
            version,
            submissions,
            challenges,
            passed_team_ids,
        ))
    }

    async fn read_epoch(&self, game_id: i64) -> Result<u64, redis::RedisError> {
        let mut conn = self.cache.connection();
        let value: Option<u64> = conn.get(self.ver_key(game_id)).await?;
        Ok(value.unwrap_or(0))
    }

    async fn read_snapshot(
        &self,
        game_id: i64,
    ) -> Result<Option<GameScoreSnapshot>, anyhow::Error> {
        let mut conn = self.cache.connection();
        let bytes: Option<Vec<u8>> = conn.hget(self.snap_key(game_id), "data").await?;
        match bytes {
            Some(bytes) => Ok(Some(GameScoreSnapshot::decode(&bytes)?)),
            None => Ok(None),
        }
    }

    async fn write_snapshot_cas(
        &self,
        game_id: i64,
        snapshot: &GameScoreSnapshot,
    ) -> Result<bool, anyhow::Error> {
        let bytes = snapshot.encode()?;
        let mut conn = self.cache.connection();
        let stored: i32 = redis::Script::new(STORE_SNAPSHOT_SCRIPT)
            .key(self.snap_key(game_id))
            .arg(snapshot.version)
            .arg(bytes)
            .arg(SNAPSHOT_TTL_SECS)
            .invoke_async(&mut conn)
            .await?;
        Ok(stored == 1)
    }

    async fn try_acquire_lock(&self, game_id: i64, token: &str) -> Result<bool, redis::RedisError> {
        let mut conn = self.cache.connection();
        let options = redis::SetOptions::default()
            .conditional_set(redis::ExistenceCheck::NX)
            .with_expiration(redis::SetExpiry::PX(REBUILD_LOCK_TTL_MS));
        let res: Option<String> = conn
            .set_options(self.lock_key(game_id), token, options)
            .await?;
        Ok(res.is_some())
    }

    async fn release_lock(&self, game_id: i64, token: &str) {
        let mut conn = self.cache.connection();
        let _ = redis::Script::new(RELEASE_LOCK_SCRIPT)
            .key(self.lock_key(game_id))
            .arg(token)
            .invoke_async::<i32>(&mut conn)
            .await;
    }
}

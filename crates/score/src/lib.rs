//! CQRS read-model score engine backed by two-tier caching (in-process L1
//! `Arc<GameScoreSnapshot>` + Valkey L2 epoch-versioned MessagePack snapshot).
//!
//! # Consistency without database write locks
//!
//! - PostgreSQL holds only immutable ground-truth facts (`submissions`,
//!   `game_challenges`, `teams.state`). Scores and ranks are **never** written
//!   back to those tables.
//! - After any score-affecting PostgreSQL commit, writers call
//!   [`ScoreService::invalidate`], which increments a monotonic Valkey epoch
//!   (`INCR score:ver:{game_id}`) and broadcasts `game_id` over NATS Core
//!   (`cds.score.updated`) so every API pod marks its L1 cache dirty within
//!   sub-milliseconds.
//! - Rebuilds always read `captured_ver = GET score:ver:{game_id}` **before**
//!   querying PostgreSQL, then commit the snapshot to Valkey via an atomic Lua
//!   CAS script (`my_ver >= snap_ver`). Therefore:
//!   1. A slower older rebuild can never overwrite a newer snapshot.
//!   2. Any DB commit concurrent with a rebuild increments `ver` *after*
//!      `captured_ver`, guaranteeing the next reader sees `snap_ver < cur_ver`
//!      and triggers a follow-up rebuild.
//! - A short cluster lock (`SET NX PX`) provides single-flight stampede
//!   protection; concurrent readers receive stale-while-revalidate (SWR)
//!   responses with sub-millisecond latency. If Valkey is temporarily
//!   unavailable, reads fall back directly to PostgreSQL without failing.

pub mod math;
pub mod snapshot;

use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use cds_cache::Cache;
use cds_db::{DB, game_challenge, submission, team};
use cds_queue::Queue;
use dashmap::DashMap;
use futures_util::StreamExt as _;
use redis::AsyncCommands;
use tokio::sync::Mutex;
use tracing::{debug, info, warn};

pub use crate::snapshot::{ChallengeScore, GameScoreSnapshot, RankedTeam, SubmissionScore};

/// NATS Core broadcast subject carrying the decimal `game_id` whenever a
/// game's score epoch advances.
pub const SCORE_UPDATED_SUBJECT: &str = "cds.score.updated";

/// Valkey TTL for cached score snapshots (recomputed automatically on miss).
const SNAPSHOT_TTL_SECS: u64 = 600;

/// Cluster-wide single-flight rebuild lock TTL.
const REBUILD_LOCK_TTL_MS: u64 = 3_000;

/// Maximum window an L1 snapshot is served without re-checking Valkey's epoch
/// when no NATS invalidation has arrived.
const L1_FRESH_TTL: Duration = Duration::from_secs(1);

/// Minimum spacing between consecutive cluster rebuilds *only* while an active
/// solve storm (`current_ver >= l2_ver + STORM_VER_THRESHOLD`) is in progress.
/// Single solves rebuild with zero artificial delay.
const STORM_COOLDOWN_MS: i64 = 200;
const STORM_VER_THRESHOLD: u64 = 3;

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

struct LocalEntry {
    snapshot: Arc<GameScoreSnapshot>,
    verified_at: Instant,
    dirty: AtomicBool,
}

struct Inner {
    db: DB,
    cache: Cache,
    queue: Queue,
    /// Per-game L1 cache holding the most recent verified snapshot.
    local: DashMap<i64, Arc<LocalEntry>>,
    /// Per-game in-process mutex collapsing concurrent misses on the same pod.
    local_locks: DashMap<i64, Arc<Mutex<()>>>,
}

/// Cloneable handle to the CQRS score service shared by Axum handlers and the
/// checker worker.
#[derive(Clone)]
pub struct ScoreService {
    inner: Arc<Inner>,
}

impl ScoreService {
    /// Creates a new [`ScoreService`] and spawns its lightweight NATS Core
    /// invalidation listener so L1 caches across horizontally scaled pods stay
    /// coherent within milliseconds.
    pub fn new(db: DB, cache: Cache, queue: Queue) -> Self {
        let service = Self {
            inner: Arc::new(Inner {
                db,
                cache: cache.scoped("score"),
                queue,
                local: DashMap::new(),
                local_locks: DashMap::new(),
            }),
        };
        service.spawn_invalidation_listener();
        service
    }

    /// Creates a [`ScoreService`] without spawning the background NATS
    /// listener (handy for isolated unit/integration tests).
    pub fn new_without_listener(db: DB, cache: Cache, queue: Queue) -> Self {
        Self {
            inner: Arc::new(Inner {
                db,
                cache: cache.scoped("score"),
                queue,
                local: DashMap::new(),
                local_locks: DashMap::new(),
            }),
        }
    }

    fn ver_key(&self, game_id: i64) -> String {
        self.inner.cache.key(format!("ver:{game_id}"))
    }

    fn snap_key(&self, game_id: i64) -> String {
        self.inner.cache.key(format!("snap:{game_id}"))
    }

    fn lock_key(&self, game_id: i64) -> String {
        self.inner.cache.key(format!("lock:{game_id}"))
    }

    fn spawn_invalidation_listener(&self) {
        let inner = Arc::clone(&self.inner);
        tokio::spawn(async move {
            loop {
                match inner.queue.subscribe_core(SCORE_UPDATED_SUBJECT).await {
                    Ok(mut sub) => {
                        info!(
                            subject = SCORE_UPDATED_SUBJECT,
                            "score invalidation listener subscribed"
                        );
                        while let Some(msg) = sub.next().await {
                            if let Ok(text) = std::str::from_utf8(&msg.payload)
                                && let Ok(game_id) = text.trim().parse::<i64>()
                            {
                                if let Some(entry) = inner.local.get(&game_id) {
                                    entry.dirty.store(true, Ordering::Release);
                                }
                            }
                        }
                        warn!("score invalidation stream ended; reconnecting");
                    }
                    Err(error) => {
                        warn!(?error, "score invalidation subscribe failed; retrying");
                    }
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
    }

    /// Marks `game_id`'s score projection dirty after a committed PostgreSQL
    /// write.
    ///
    /// Never writes to PostgreSQL and never fails the caller: if Valkey or NATS
    /// is transiently unavailable, L1 expiry and Valkey TTL self-heal.
    pub async fn invalidate(&self, game_id: i64) {
        if let Some(entry) = self.inner.local.get(&game_id) {
            entry.dirty.store(true, Ordering::Release);
        }

        let mut conn = self.inner.cache.connection();
        match conn.incr::<_, _, u64>(self.ver_key(game_id), 1_u64).await {
            Ok(ver) => {
                debug!(game_id, version = ver, "score epoch incremented");
            }
            Err(error) => {
                warn!(game_id, ?error, "score epoch increment failed");
            }
        }

        if let Err(error) = self
            .inner
            .queue
            .notify(SCORE_UPDATED_SUBJECT, game_id.to_string())
            .await
        {
            warn!(game_id, ?error, "score invalidation broadcast failed");
        }
    }

    /// Forces an immediate recomputation of `game_id` from PostgreSQL,
    /// bypassing any burst cooldown, and publishes the fresh snapshot to
    /// Valkey and L1.
    pub async fn force_refresh(
        &self,
        game_id: i64,
    ) -> Result<Arc<GameScoreSnapshot>, anyhow::Error> {
        self.invalidate(game_id).await;
        let local_lock = self.per_game_lock(game_id);
        let _guard = local_lock.lock().await;
        self.rebuild_and_store(game_id).await
    }

    /// Returns the latest score snapshot for `game_id`, serving from L1 RAM or
    /// L2 Valkey whenever fresh and transparently recomputing on miss or epoch
    /// advance.
    pub async fn snapshot(&self, game_id: i64) -> Result<Arc<GameScoreSnapshot>, anyhow::Error> {
        // 1. Hot path: clean L1 hit within L1_FRESH_TTL (zero network I/O).
        if let Some(entry) = self.inner.local.get(&game_id)
            && !entry.dirty.load(Ordering::Acquire)
            && entry.verified_at.elapsed() < L1_FRESH_TTL
        {
            return Ok(Arc::clone(&entry.snapshot));
        }

        // 2. Read current Valkey epoch (10 bytes). Fall back to PG on cache
        //    outage so score endpoints stay resilient.
        let current_ver = match self.read_epoch(game_id).await {
            Ok(ver) => ver,
            Err(error) => {
                warn!(
                    game_id,
                    ?error,
                    "valkey epoch lookup failed; computing snapshot directly from PostgreSQL"
                );
                let local_lock = self.per_game_lock(game_id);
                let _guard = local_lock.lock().await;
                let snap = Arc::new(self.compute_from_db(game_id, 0).await?);
                self.store_local(game_id, Arc::clone(&snap));
                return Ok(snap);
            }
        };

        // 3. L1 still matches the latest epoch -> renew verification timestamp
        //    without transferring the snapshot payload over the network.
        if let Some(entry) = self.inner.local.get(&game_id)
            && entry.snapshot.version == current_ver
            && !entry.dirty.load(Ordering::Acquire)
        {
            let snap = Arc::clone(&entry.snapshot);
            drop(entry);
            self.store_local(game_id, Arc::clone(&snap));
            return Ok(snap);
        }

        // 4. Collapse concurrent misses on the same pod.
        let local_lock = self.per_game_lock(game_id);
        let _guard = local_lock.lock().await;

        // Re-check L1 after acquiring local lock in case a sibling task filled
        // it.
        if let Some(entry) = self.inner.local.get(&game_id)
            && entry.snapshot.version == current_ver
            && !entry.dirty.load(Ordering::Acquire)
        {
            return Ok(Arc::clone(&entry.snapshot));
        }

        // 5. Try L2 Valkey snapshot.
        let l2 = match self.read_l2_snapshot(game_id).await {
            Ok(snap) => snap,
            Err(error) => {
                warn!(game_id, ?error, "valkey snapshot read failed");
                None
            }
        };

        if let Some(ref snap) = l2 {
            if snap.version >= current_ver {
                self.store_local(game_id, Arc::clone(snap));
                return Ok(Arc::clone(snap));
            }

            // Under a rapid solve flood (>= 3 invalidations within 200ms),
            // serve the previous snapshot immediately (SWR) and schedule one
            // debounced background refresh so PG rebuild rate stays bounded.
            let age_ms = now_unix_ms().saturating_sub(snap.computed_at_ms);
            if current_ver.saturating_sub(snap.version) >= STORM_VER_THRESHOLD
                && age_ms < STORM_COOLDOWN_MS
            {
                self.store_local(game_id, Arc::clone(snap));
                self.spawn_debounced_refresh(
                    game_id,
                    Duration::from_millis((STORM_COOLDOWN_MS - age_ms).max(10) as u64),
                );
                return Ok(Arc::clone(snap));
            }
        }

        // 6. Cluster single-flight rebuild lock.
        let token = nanoid::nanoid!();
        let acquired = self.try_acquire_lock(game_id, &token).await.unwrap_or(true);
        if !acquired {
            // Another pod is actively rebuilding. Serve stale snapshot via SWR
            // if available; on a cold cache wait briefly for the builder to
            // finish before computing locally.
            if let Some(stale) = l2.or_else(|| self.local_snapshot(game_id)) {
                return Ok(stale);
            }
            tokio::time::sleep(Duration::from_millis(25)).await;
            if let Ok(Some(fresh)) = self.read_l2_snapshot(game_id).await {
                self.store_local(game_id, Arc::clone(&fresh));
                return Ok(fresh);
            }
            let snap = Arc::new(self.compute_from_db(game_id, current_ver).await?);
            self.store_local(game_id, Arc::clone(&snap));
            return Ok(snap);
        }

        let result = self.rebuild_and_store(game_id).await;
        self.release_lock(game_id, &token).await;
        result
    }

    /// Reads `captured_ver` **before** querying PostgreSQL, builds the
    /// snapshot, persists it to Valkey via Lua CAS, and updates L1.
    async fn rebuild_and_store(
        &self,
        game_id: i64,
    ) -> Result<Arc<GameScoreSnapshot>, anyhow::Error> {
        let started = Instant::now();
        let captured_ver = self.read_epoch(game_id).await.unwrap_or(0);
        let snap = Arc::new(self.compute_from_db(game_id, captured_ver).await?);

        if let Err(error) = self.write_l2_snapshot_cas(game_id, &snap).await {
            warn!(game_id, ?error, "valkey snapshot CAS write failed");
        }
        self.store_local(game_id, Arc::clone(&snap));

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
            submission::find_score_inputs(&self.inner.db.conn, game_id),
            game_challenge::find_score_inputs(&self.inner.db.conn, game_id),
            team::find_passed_ids(&self.inner.db.conn, game_id),
        )?;

        Ok(snapshot::build(
            version,
            now_unix_ms(),
            submissions,
            challenges,
            passed_team_ids,
        ))
    }

    fn per_game_lock(&self, game_id: i64) -> Arc<Mutex<()>> {
        self.inner
            .local_locks
            .entry(game_id)
            .or_insert_with(|| Arc::new(Mutex::new(())))
            .clone()
    }

    fn local_snapshot(&self, game_id: i64) -> Option<Arc<GameScoreSnapshot>> {
        self.inner
            .local
            .get(&game_id)
            .map(|entry| Arc::clone(&entry.snapshot))
    }

    fn store_local(&self, game_id: i64, snapshot: Arc<GameScoreSnapshot>) {
        self.inner.local.insert(
            game_id,
            Arc::new(LocalEntry {
                snapshot,
                verified_at: Instant::now(),
                dirty: AtomicBool::new(false),
            }),
        );
    }

    fn spawn_debounced_refresh(&self, game_id: i64, delay: Duration) {
        let service = self.clone();
        tokio::spawn(async move {
            tokio::time::sleep(delay).await;
            let token = nanoid::nanoid!();
            if service
                .try_acquire_lock(game_id, &token)
                .await
                .unwrap_or(false)
            {
                if let Err(error) = service.rebuild_and_store(game_id).await {
                    warn!(game_id, ?error, "debounced score refresh failed");
                }
                service.release_lock(game_id, &token).await;
            }
        });
    }

    async fn read_epoch(&self, game_id: i64) -> Result<u64, redis::RedisError> {
        let mut conn = self.inner.cache.connection();
        let value: Option<u64> = conn.get(self.ver_key(game_id)).await?;
        Ok(value.unwrap_or(0))
    }

    async fn read_l2_snapshot(
        &self,
        game_id: i64,
    ) -> Result<Option<Arc<GameScoreSnapshot>>, anyhow::Error> {
        let mut conn = self.inner.cache.connection();
        let bytes: Option<Vec<u8>> = conn.hget(self.snap_key(game_id), "data").await?;
        match bytes {
            Some(bytes) => Ok(Some(Arc::new(GameScoreSnapshot::decode(&bytes)?))),
            None => Ok(None),
        }
    }

    async fn write_l2_snapshot_cas(
        &self,
        game_id: i64,
        snapshot: &GameScoreSnapshot,
    ) -> Result<bool, anyhow::Error> {
        let bytes = snapshot.encode()?;
        let mut conn = self.inner.cache.connection();
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
        let mut conn = self.inner.cache.connection();
        let options = redis::SetOptions::default()
            .conditional_set(redis::ExistenceCheck::NX)
            .with_expiration(redis::SetExpiry::PX(REBUILD_LOCK_TTL_MS));
        let res: Option<String> = conn
            .set_options(self.lock_key(game_id), token, options)
            .await?;
        Ok(res.is_some())
    }

    async fn release_lock(&self, game_id: i64, token: &str) {
        let mut conn = self.inner.cache.connection();
        let _ = redis::Script::new(RELEASE_LOCK_SCRIPT)
            .key(self.lock_key(game_id))
            .arg(token)
            .invoke_async::<i32>(&mut conn)
            .await;
    }
}

fn now_unix_ms() -> i64 {
    (time::OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as i64
}

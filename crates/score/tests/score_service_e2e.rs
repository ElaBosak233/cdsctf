//! Real PostgreSQL + Valkey + NATS integration and 2,000-team scale tests for
//! [`cds_score::ScoreService`].

use std::time::{Duration, Instant};

use cds_cache::Cache;
use cds_db::{
    DB,
    sea_orm::{ConnectOptions, ConnectionTrait, Database, DbBackend, Statement},
    team::FindTeamOptions,
};
use cds_score::{ScoreService, math};

async fn connect_infra(namespace: &str) -> (DB, Cache, cds_queue::Queue) {
    let db_url = std::env::var("CDS_TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://cdsctf:cdsctf@127.0.0.1:15432/cdsctf".to_owned());
    let cache_url = std::env::var("CDS_TEST_CACHE_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1:16379/0".to_owned());
    let nats_port: u16 = std::env::var("CDS_TEST_NATS_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(14222);

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(16)
        .min_connections(2)
        .sqlx_logging(false);
    let db = DB {
        conn: Database::connect(opt).await.unwrap(),
    };
    cds_migrator::run(&db).await.unwrap();

    let cache = Cache::connect(
        &cache_url,
        namespace,
        Duration::from_secs(2),
        Duration::from_secs(2),
        64,
    )
    .await
    .unwrap();
    let _ = cache.clear_namespace().await;

    let mut env = cds_env::Env::default();
    env.queue.host = "127.0.0.1".to_owned();
    env.queue.port = nats_port;
    let queue = cds_queue::init(&env).await.unwrap();

    (db, cache, queue)
}

#[tokio::test]
#[ignore = "requires disposable PG (15432), Valkey (16379), NATS (14222)"]
async fn multi_pod_epoch_invalidation_and_valkey_flush_recovery() {
    let ns = format!("cds-score-e2e-{}", std::process::id());
    let (db, cache, queue) = connect_infra(&ns).await;

    // Seed an isolated game in PG (high IDs to avoid colliding with other
    // tests).
    let game_id = 910_001_i64;
    db.conn
        .execute_unprepared(&format!(
            r#"
                DELETE FROM games WHERE id = {game_id};
                DELETE FROM challenges WHERE id IN (910010, 910011);
                DELETE FROM users WHERE id IN (910001, 910002);

                INSERT INTO users (id, name, username, hashed_password, created_at, updated_at)
                VALUES
                    (910001, 'Alice', 'score-alice', 'x', now(), now()),
                    (910002, 'Bob', 'score-bob', 'x', now(), now());
                INSERT INTO challenges (
                    id, title, description, category, tags, has_instance,
                    has_attachment, public, has_writeup, created_at, updated_at
                ) VALUES
                    (910010, 'Web 1', '', 1, '{{}}', false, false, true, false, now(), now()),
                    (910011, 'Pwn 1', '', 2, '{{}}', false, false, true, false, now(), now());
                INSERT INTO games (
                    id, title, enabled, public, timeslots, started_at, frozen_at, ended_at, created_at
                ) VALUES (
                    {game_id}, 'MultiPod Game', true, true, '[]',
                    now() - interval '1 hour', now() + interval '1 hour', now() + interval '2 hour', now()
                );
                INSERT INTO game_challenges (game_id, challenge_id, difficulty, max_pts, min_pts, bonus_ratios, enabled)
                VALUES
                    ({game_id}, 910010, 10, 1000, 100, ARRAY[10, 5, 0]::BIGINT[], true),
                    ({game_id}, 910011, 10, 1000, 100, ARRAY[10, 5, 0]::BIGINT[], true);
                INSERT INTO teams (id, game_id, name, state) VALUES
                    (910101, {game_id}, 'Alpha', 3),
                    (910102, {game_id}, 'Beta', 3);
                INSERT INTO submissions (id, content, status, challenge_id, user_id, team_id, game_id, created_at)
                VALUES
                    (910501, 'flag', 'correct', 910010, 910001, 910101, {game_id}, to_timestamp(1000));
            "#
        ))
        .await
        .unwrap();

    // Two independent ScoreService instances simulating two horizontally scaled
    // API pods.
    let pod_a = ScoreService::new(db.clone(), cache.clone(), queue.clone());
    let pod_b = ScoreService::new(db.clone(), cache.clone(), queue.clone());
    tokio::time::sleep(Duration::from_millis(100)).await;

    let snap_a1 = pod_a.snapshot(game_id).await.unwrap();
    assert_eq!(snap_a1.team_score(910101), (1100, 1));
    assert_eq!(snap_a1.team_score(910102), (0, 2));

    // Pod B commits a new solve by Beta and invalidates the epoch.
    db.conn
        .execute_unprepared(&format!(
            r#"
                INSERT INTO submissions (id, content, status, challenge_id, user_id, team_id, game_id, created_at)
                VALUES
                    (910502, 'flag', 'correct', 910010, 910002, 910102, {game_id}, to_timestamp(1010)),
                    (910503, 'flag', 'correct', 910011, 910002, 910102, {game_id}, to_timestamp(1020));
            "#
        ))
        .await
        .unwrap();
    pod_b.invalidate(game_id).await;
    // Allow NATS Core invalidation broadcast to reach Pod A's L1 cache.
    tokio::time::sleep(Duration::from_millis(50)).await;

    let snap_a2 = pod_a.snapshot(game_id).await.unwrap();
    let c1_base = math::curve(1000, 100, 10, 2);
    let c2_base = math::curve(1000, 100, 10, 1);
    let expected_alpha = c1_base * 110 / 100;
    let expected_beta = c1_base * 105 / 100 + c2_base * 110 / 100;
    assert_eq!(snap_a2.team_score(910102), (expected_beta, 1));
    assert_eq!(snap_a2.team_score(910101), (expected_alpha, 2));
    assert!(snap_a2.version > snap_a1.version);

    // Simulate banning Alpha -> Alpha's solve is excluded and Beta inherits 1st
    // blood on c1!
    db.conn
        .execute_unprepared("UPDATE teams SET state = 0 WHERE id = 910101")
        .await
        .unwrap();
    pod_b.invalidate(game_id).await;
    tokio::time::sleep(Duration::from_millis(50)).await;

    let snap_a3 = pod_a.snapshot(game_id).await.unwrap();
    assert_eq!(snap_a3.ranked_teams.len(), 1);
    assert_eq!(snap_a3.team_score(910101), (0, 0));
    assert_eq!(snap_a3.challenges[&910010].solve_count, 1);
    assert_eq!(
        snap_a3.challenges[&910010].blood_submission_ids,
        vec![910502]
    );
    assert_eq!(snap_a3.team_score(910102), (1100 + 1100, 1));

    // Simulate Valkey restart/flush -> transparent automatic rebuild from
    // PostgreSQL.
    cache.clear_namespace().await.unwrap();
    let snap_recovered = pod_b.force_refresh(game_id).await.unwrap();
    assert_eq!(snap_recovered.team_score(910102), (2200, 1));

    // Cleanup.
    db.conn
        .execute_unprepared(&format!(
            "DELETE FROM games WHERE id = {game_id}; DELETE FROM challenges WHERE id IN (910010, 910011); DELETE FROM users WHERE id IN (910001, 910002);"
        ))
        .await
        .unwrap();
    let _ = cache.clear_namespace().await;
}

#[tokio::test]
#[ignore = "requires disposable PG (15432), Valkey (16379), NATS (14222)"]
async fn scales_to_2000_teams_and_10000_solves_on_real_postgres_and_valkey() {
    let ns = format!("cds-score-scale-{}", std::process::id());
    let (db, cache, queue) = connect_infra(&ns).await;

    let game_id = 920_001_i64;
    db.conn
        .execute_unprepared(&format!(
            r#"
                DELETE FROM games WHERE id = {game_id};
                DELETE FROM challenges WHERE id BETWEEN 920001 AND 920020;
                DELETE FROM users WHERE id = 920001;

                INSERT INTO users (id, name, username, hashed_password, created_at, updated_at)
                VALUES (920001, 'ScaleUser', 'scale-user', 'x', now(), now());

                INSERT INTO challenges (
                    id, title, description, category, tags, has_instance,
                    has_attachment, public, has_writeup, created_at, updated_at
                )
                SELECT 920000 + i, 'Challenge-' || i, '', (i % 5)::int, '{{}}', false, false, true, false, now(), now()
                FROM generate_series(1, 20) AS i;

                INSERT INTO games (
                    id, title, enabled, public, timeslots, started_at, frozen_at, ended_at, created_at
                ) VALUES (
                    {game_id}, '2000-Team Scale CTF', true, true, '[]',
                    now() - interval '1 hour', now() + interval '1 hour', now() + interval '2 hour', now()
                );

                INSERT INTO game_challenges (game_id, challenge_id, difficulty, max_pts, min_pts, bonus_ratios, enabled)
                SELECT {game_id}, 920000 + i, 25, 1000, 100, ARRAY[10, 5, 2]::BIGINT[], true
                FROM generate_series(1, 20) AS i;

                -- 2,000 Passed teams + 100 Pending teams = 2,100 total teams.
                INSERT INTO teams (id, game_id, name, state)
                SELECT 9200000 + i, {game_id}, 'Team-' || i, 3
                FROM generate_series(1, 2000) AS i;

                INSERT INTO teams (id, game_id, name, state)
                SELECT 9202000 + i, {game_id}, 'Pending-' || i, 2
                FROM generate_series(1, 100) AS i;

                -- 10,000 unique correct solves (5 distinct challenges per team on average).
                INSERT INTO submissions (id, content, status, challenge_id, user_id, team_id, game_id, created_at)
                SELECT
                    92000000 + ((t - 1) * 5 + k),
                    'flag{{scale}}',
                    'correct',
                    920001 + (((t + k * 3) % 20)),
                    920001,
                    9200000 + t,
                    {game_id},
                    to_timestamp(1700000000 + t * 2 + k)
                FROM generate_series(1, 2000) AS t
                CROSS JOIN generate_series(0, 4) AS k;

                -- 40,000 incorrect submissions to mirror real CTF noise (80% wrong).
                INSERT INTO submissions (id, content, status, challenge_id, user_id, team_id, game_id, created_at)
                SELECT
                    92100000 + i,
                    'flag{{wrong}}',
                    'incorrect',
                    920001 + (i % 20),
                    920001,
                    9200001 + (i % 2000),
                    {game_id},
                    to_timestamp(1700000000 + i)
                FROM generate_series(1, 40000) AS i;

                ANALYZE submissions;
                ANALYZE teams;
                ANALYZE game_challenges;
            "#
        ))
        .await
        .unwrap();
    db.conn
        .execute_unprepared("VACUUM ANALYZE submissions;")
        .await
        .unwrap();

    let service = ScoreService::new_without_listener(db.clone(), cache.clone(), queue.clone());

    // 1. Cold rebuild (PG Index-Only Scan + Rust compute + Valkey MessagePack
    //    CAS)
    let t_cold = Instant::now();
    let snap = service.snapshot(game_id).await.unwrap();
    let cold_ms = t_cold.elapsed().as_secs_f64() * 1000.0;
    let payload_kb = snap.encode().unwrap().len() as f64 / 1024.0;

    assert_eq!(snap.ranked_teams.len(), 2_000);
    assert_eq!(snap.submission_scores.len(), 10_000);
    assert_eq!(snap.ranked_teams[0].rank, 1);
    assert_eq!(snap.ranked_teams[1_999].rank, 2_000);

    // 2. L1 warm lookup
    let t_l1 = Instant::now();
    let snap_l1 = service.snapshot(game_id).await.unwrap();
    let l1_us = t_l1.elapsed().as_secs_f64() * 1_000_000.0;
    assert_eq!(snap_l1.team_score(snap.ranked_teams[0].team_id).1, 1);

    // 3. L2 Valkey hit from a fresh second pod
    let pod2 = ScoreService::new_without_listener(db.clone(), cache.clone(), queue.clone());
    let t_l2 = Instant::now();
    let snap_l2 = pod2.snapshot(game_id).await.unwrap();
    let l2_ms = t_l2.elapsed().as_secs_f64() * 1000.0;
    assert_eq!(snap_l2.ranked_teams.len(), 2_000);

    // 4. Scoreboard pagination (page 1 & last page 34, size 60)
    let t_page1 = Instant::now();
    let (p1_teams, total) = snap.page_teams(Some(1), Some(60));
    let p1_ids: Vec<i64> = p1_teams.iter().map(|t| t.team_id).collect();
    let p1_entries = cds_db::team::find_scoreboard_entries_by_ids(&db.conn, game_id, &p1_ids)
        .await
        .unwrap();
    let page1_ms = t_page1.elapsed().as_secs_f64() * 1000.0;
    assert_eq!(total, 2_000);
    assert_eq!(p1_entries.len(), 60);

    let (p34_teams, _) = snap.page_teams(Some(34), Some(60));
    assert_eq!(p34_teams.len(), 20); // 33 * 60 + 20 = 2000

    // 5. Admin filter-then-rank across 2,100 teams (name LIKE '%Team-1%' + rank
    //    sort)
    let t_admin = Instant::now();
    let mut matched = cds_db::team::find_ids(
        &db.conn,
        &FindTeamOptions {
            game_id: Some(game_id),
            name: Some("Team-1".to_owned()),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    matched.sort_unstable_by_key(|&id| {
        let (_, r) = snap.team_score(id);
        (r == 0, r, id)
    });
    let admin_page = cds_db::team::find_with_users_by_ids(&db.conn, game_id, &matched[..20])
        .await
        .unwrap();
    let admin_filter_ms = t_admin.elapsed().as_secs_f64() * 1000.0;
    assert_eq!(admin_page.len(), 20);
    assert!(matched.len() > 1_000); // Team-1, Team-10..19, Team-100..199, Team-1000..1999 = 1,111 teams

    // Verify EXPLAIN uses our partial covering index `idx_submissions_scoring`.
    let plan_rows = db
        .conn
        .query_all_raw(Statement::from_string(
            DbBackend::Postgres,
            format!(
                "EXPLAIN SELECT id, challenge_id, team_id, created_at FROM submissions WHERE game_id = {game_id} AND status = 'correct'"
            ),
        ))
        .await
        .unwrap();
    let plan = plan_rows
        .into_iter()
        .map(|r| r.try_get::<String>("", "QUERY PLAN").unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        plan.contains("idx_submissions_scoring") || plan.contains("uq_submissions_correct_game"),
        "expected index scan on submissions, got:\n{plan}"
    );

    println!(
        "SCALE_METRICS_2000_TEAMS cold_rebuild_ms={cold_ms:.2} payload_kb={payload_kb:.1} l1_hit_us={l1_us:.1} l2_hit_ms={l2_ms:.2} scoreboard_page1_ms={page1_ms:.2} admin_filter_rank_ms={admin_filter_ms:.2}"
    );

    // Cleanup.
    db.conn
        .execute_unprepared(&format!(
            "DELETE FROM games WHERE id = {game_id}; DELETE FROM challenges WHERE id BETWEEN 920001 AND 920020; DELETE FROM users WHERE id = 920001;"
        ))
        .await
        .unwrap();
    let _ = cache.clear_namespace().await;
}

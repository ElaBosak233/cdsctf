//! Full-stack end-to-end API compatibility & multi-worker judging test.
//!
//! Exercises the **actual Axum HTTP routes** (`POST /api/submissions`,
//! `GET /api/submissions`, `GET /api/games/{id}/scoreboard`,
//! `GET /api/games/{id}/challenges`, `POST /api/challenges/status`,
//! `GET /api/games/{id}/teams/us`, `POST /api/games/{id}/teams/us/ready`,
//! `GET /api/management/games/{id}/teams`, `GET /api/management/submissions`,
//! `PATCH /api/management/submissions/{id}/status`,
//! `POST /api/management/games/{id}/calculate`,
//! `PATCH /api/management/games/{id}/challenges/{cid}`) alongside **two
//! concurrent `cds_worker::checker` workers**, real PostgreSQL, real Valkey,
//! and real NATS.

use std::{sync::Arc, time::Duration};

use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Method, Request, StatusCode},
    routing::any,
};
use cds_cache::Cache;
use cds_db::{
    DB, UserAccountView,
    sea_orm::{ConnectOptions, ConnectionTrait, Database, DbBackend, Statement},
};
use cds_score::{ScoreService, math};
use cds_web::traits::{AppState, AuthMethod, AuthPrincipal};
use serde_json::{Value, json};
use tokio::net::TcpListener;
use tower::ServiceExt;

const TEST_KEY: &str = "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111";

/// Tiny in-process S3 mock returning `TEST_KEY` for `.key` requests and `200
/// OK` for bucket checks so `cds_checker` works without an external S3 daemon.
async fn start_mock_s3() -> String {
    let app = Router::new().fallback(any(|req: Request<Body>| async move {
        let method = req.method().clone();
        let path = req.uri().path().to_owned();
        if method == Method::HEAD {
            return (StatusCode::OK, Body::empty());
        }
        if path.ends_with("/.key") {
            return (StatusCode::OK, Body::from(TEST_KEY));
        }
        (
            StatusCode::OK,
            Body::from(
                r#"<?xml version="1.0" encoding="UTF-8"?><ListAllMyBucketsResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/"><Owner><ID>test</ID><DisplayName>test</DisplayName></Owner><Buckets><Bucket><Name>cdsctf-e2e</Name><CreationDate>2026-01-01T00:00:00Z</CreationDate></Bucket></Buckets></ListAllMyBucketsResult>"#,
            ),
        )
    }));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });
    format!("http://{addr}")
}

fn principal(user: UserAccountView) -> AuthPrincipal {
    AuthPrincipal {
        operator: Some(user),
        client_ip: "127.0.0.1".to_owned(),
        auth_method: AuthMethod::Session,
    }
}

async fn send_json(
    app: &Router,
    method: Method,
    uri: &str,
    auth: Option<AuthPrincipal>,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder().method(method).uri(uri);
    let body = if let Some(json_body) = body {
        builder = builder.header("content-type", "application/json");
        Body::from(serde_json::to_vec(&json_body).unwrap())
    } else {
        Body::empty()
    };
    let mut req = builder.body(body).unwrap();
    if let Some(auth) = auth {
        req.extensions_mut().insert(auth);
    }
    let resp = app.clone().oneshot(req).await.unwrap();
    let status = resp.status();
    let bytes = to_bytes(resp.into_body(), 4 * 1024 * 1024).await.unwrap();
    let json = serde_json::from_slice::<Value>(&bytes)
        .unwrap_or_else(|_| json!({ "_raw": String::from_utf8_lossy(&bytes) }));
    (status, json)
}

async fn wait_until_terminal(db: &DB, ids: &[i64]) {
    let list = ids
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",");
    tokio::time::timeout(Duration::from_secs(15), async {
        loop {
            let pending: i64 = db
                .conn
                .query_one_raw(Statement::from_string(
                    DbBackend::Postgres,
                    format!(
                        "SELECT COUNT(*) AS c FROM submissions WHERE id IN ({list}) AND status IN ('queued', 'processing')"
                    ),
                ))
                .await
                .unwrap()
                .unwrap()
                .try_get("", "c")
                .unwrap();
            if pending == 0 {
                return;
            }
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    })
    .await
    .expect("submissions did not finish judging in time");
}

#[tokio::test]
#[ignore = "requires disposable PG (15432), Valkey (16379), NATS (14222)"]
async fn full_judging_and_scoring_api_compatibility_e2e() {
    let _ = rustls::crypto::ring::default_provider().install_default();
    let s3_endpoint = start_mock_s3().await;

    let mut env = cds_env::Env::default();
    env.queue.host = "127.0.0.1".to_owned();
    env.queue.port = 14222;
    env.media.endpoint = s3_endpoint;
    env.media.bucket = "cdsctf-e2e".to_owned();
    env.media.access_key = "test".to_owned();
    env.media.secret_key = "test".to_owned();
    env.media.path_style = true;

    let db_url = std::env::var("CDS_TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://cdsctf:cdsctf@127.0.0.1:15432/cdsctf".to_owned());
    let mut db_opt = ConnectOptions::new(db_url);
    db_opt
        .max_connections(16)
        .min_connections(2)
        .sqlx_logging(false);
    let db = DB {
        conn: Database::connect(db_opt).await.unwrap(),
    };
    cds_migrator::run(&db).await.unwrap();

    let ns = format!("cds-web-e2e-{}", std::process::id());
    let cache = Cache::connect(
        "redis://127.0.0.1:16379/0",
        &ns,
        Duration::from_secs(2),
        Duration::from_secs(2),
        64,
    )
    .await
    .unwrap();
    let _ = cache.clear_namespace().await;

    let queue = cds_queue::init(&env).await.unwrap();
    let event = cds_event::init(&queue).unwrap();
    let media = cds_media::init(&env).await.unwrap();
    let checker = cds_checker::init(&media).unwrap();
    let score = ScoreService::new(db.clone(), cache.clone(), queue.clone());
    let cluster = cds_cluster::Cluster::disconnected(&checker).unwrap();
    let mailbox = cds_mailbox::Mailbox::new(db.clone(), media.clone());
    let captcha = cds_captcha::init(&db, &cache).unwrap();

    let state = Arc::new(AppState {
        env,
        event,
        db: db.clone(),
        cache: cache.clone(),
        checker: checker.clone(),
        captcha,
        cluster,
        media,
        mailbox,
        queue: queue.clone(),
        score: score.clone(),
    });

    // Spawn TWO independent checker workers to verify multi-worker SKIP LOCKED
    // concurrency and NATS Core queue-group wakeups.
    cds_worker::checker::spawn(&db, &queue, &checker, &score).await;
    cds_worker::checker::spawn(&db, &queue, &checker, &score).await;
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Mount the actual `/api` router tree (without session middleware so tests
    // inject `AuthPrincipal` cleanly, preserving exact handler + response
    // paths).
    let api: Router = cds_web::router::api::openapi_documented_under_api(Arc::clone(&state))
        .split_for_parts()
        .0
        .with_state(Arc::clone(&state));
    let app = Router::new().nest("/api", api);

    // Seed isolated test records (IDs in 930_xxx range).
    let game_id = 930_001_i64;
    let c_static = 930_010_i64;
    let c_suid = 930_011_i64;
    let c_broken = 930_012_i64;
    let u_alice = 930_101_i64;
    let u_bob = 930_102_i64;
    let u_charlie = 930_103_i64;
    let u_admin = 930_109_i64;
    let t_alpha = 930_201_i64;
    let t_beta = 930_202_i64;
    let t_charlie = 930_203_i64;
    let t_pending = 930_204_i64;

    let static_script = r#"
        function generate(subject_id) return {} end
        function check(subject_id, content)
            if content == "flag{static_ok}" then
                return checker.audit.correct()
            end
            return checker.audit.incorrect()
        end
    "#;
    let suid_script = r#"
        function generate(subject_id)
            return { FLAG = "flag{" .. checker.suid.encode(subject_id) .. "}" }
        end
        function check(subject_id, content)
            local parsed = checker.audit.parse(content)
            if not parsed then return checker.audit.incorrect() end
            local decoded = checker.suid.decode(parsed.content)
            if decoded == nil then return checker.audit.incorrect() end
            if decoded == subject_id then return checker.audit.correct() end
            return checker.audit.cheat(decoded)
        end
    "#;
    let broken_script = r#"
        function generate(subject_id) return {} end
        function check(subject_id, content)
            error("deliberate script crash")
        end
    "#;

    db.conn
        .execute_unprepared(&format!(
            r#"
                DELETE FROM games WHERE id = {game_id};
                DELETE FROM challenges WHERE id IN ({c_static}, {c_suid}, {c_broken});
                DELETE FROM users WHERE id IN ({u_alice}, {u_bob}, {u_charlie}, {u_admin});

                INSERT INTO users (id, name, username, hashed_password, permissions, created_at, updated_at)
                VALUES
                    ({u_alice},   'Alice',   'e2e-alice',   'x', ARRAY['basic'], now(), now()),
                    ({u_bob},     'Bob',     'e2e-bob',     'x', ARRAY['basic'], now(), now()),
                    ({u_charlie}, 'Charlie', 'e2e-charlie', 'x', ARRAY['basic'], now(), now()),
                    ({u_admin},   'Admin',   'e2e-admin',   'x', ARRAY['basic','management:game','management:challenge','management:user','management:config'], now(), now());

                INSERT INTO games (
                    id, title, enabled, public, paused, blacked_out, timeslots,
                    started_at, frozen_at, ended_at, created_at
                ) VALUES (
                    {game_id}, 'E2E Championship', true, true, false, false, '[]',
                    now() - interval '1 hour', now() + interval '1 hour', now() + interval '2 hour', now()
                );

                INSERT INTO teams (id, game_id, name, state) VALUES
                    ({t_alpha},   {game_id}, 'Team Alpha',   3),
                    ({t_beta},    {game_id}, 'Team Beta',    3),
                    ({t_charlie}, {game_id}, 'Team Charlie', 3),
                    ({t_pending}, {game_id}, 'Team Pending', 2);

                INSERT INTO team_users (team_id, user_id) VALUES
                    ({t_alpha},   {u_alice}),
                    ({t_beta},    {u_bob}),
                    ({t_charlie}, {u_charlie});
            "#
        ))
        .await
        .unwrap();

    for (cid, title, cat, script) in [
        (c_static, "Static Web", 1, static_script),
        (c_suid, "Dynamic Pwn", 2, suid_script),
        (c_broken, "Broken Misc", 3, broken_script),
    ] {
        db.conn
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Postgres,
                "INSERT INTO challenges (id, title, description, category, tags, has_instance, has_attachment, public, has_writeup, checker, created_at, updated_at) VALUES ($1, $2, '', $3, '{}', false, false, true, false, $4, now(), now())",
                [cid.into(), title.into(), cat.into(), script.into()],
            ))
            .await
            .unwrap();
        db.conn
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Postgres,
                "INSERT INTO game_challenges (game_id, challenge_id, difficulty, max_pts, min_pts, bonus_ratios, enabled) VALUES ($1, $2, 10, 1000, 100, ARRAY[10, 5, 0]::BIGINT[], true)",
                [game_id.into(), cid.into()],
            ))
            .await
            .unwrap();
    }

    let load_user = |uid: i64| {
        let db = db.clone();
        async move {
            cds_db::user::find_by_id::<UserAccountView>(&db.conn, uid)
                .await
                .unwrap()
                .unwrap()
        }
    };
    let alice = principal(load_user(u_alice).await);
    let bob = principal(load_user(u_bob).await);
    let charlie = principal(load_user(u_charlie).await);
    let admin = principal(load_user(u_admin).await);

    // -------------------------------------------------------------------------
    // 1. POST /api/submissions: wrong + correct + duplicate + broken script
    // -------------------------------------------------------------------------
    let (status, wrong_sub) = send_json(
        &app,
        Method::POST,
        "/api/submissions",
        Some(alice.clone()),
        Some(json!({
            "content": "flag{wrong}",
            "game_id": game_id,
            "team_id": t_alpha,
            "challenge_id": c_static
        })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(wrong_sub["status"], "queued");
    assert_eq!(wrong_sub["pts"], 0);
    assert_eq!(wrong_sub["rank"], 0);
    assert!(wrong_sub.get("content").is_none()); // redacted in SubmissionSummary
    let wrong_id = wrong_sub["id"].as_i64().unwrap();

    // Alice solves c_static first, then submits 3 duplicates; Bob solves
    // c_static second.
    let (s1, a_sub1) = send_json(
        &app,
        Method::POST,
        "/api/submissions",
        Some(alice.clone()),
        Some(json!({
            "content": "flag{static_ok}",
            "game_id": game_id,
            "team_id": t_alpha,
            "challenge_id": c_static
        })),
    )
    .await;
    assert_eq!(s1, StatusCode::CREATED);
    let a_sub1_id = a_sub1["id"].as_i64().unwrap();
    wait_until_terminal(&db, &[wrong_id, a_sub1_id]).await;

    let mut dup_ids = Vec::new();
    for _ in 0..3 {
        let (st, body) = send_json(
            &app,
            Method::POST,
            "/api/submissions",
            Some(alice.clone()),
            Some(json!({
                "content": "flag{static_ok}",
                "game_id": game_id,
                "team_id": t_alpha,
                "challenge_id": c_static
            })),
        )
        .await;
        assert_eq!(st, StatusCode::CREATED);
        dup_ids.push(body["id"].as_i64().unwrap());
    }

    let (s2, b_sub1) = send_json(
        &app,
        Method::POST,
        "/api/submissions",
        Some(bob.clone()),
        Some(json!({
            "content": "flag{static_ok}",
            "game_id": game_id,
            "team_id": t_beta,
            "challenge_id": c_static
        })),
    )
    .await;
    assert_eq!(s2, StatusCode::CREATED);
    let b_sub1_id = b_sub1["id"].as_i64().unwrap();

    let (s3, broken_sub) = send_json(
        &app,
        Method::POST,
        "/api/submissions",
        Some(bob.clone()),
        Some(json!({
            "content": "flag{anything}",
            "game_id": game_id,
            "team_id": t_beta,
            "challenge_id": c_broken
        })),
    )
    .await;
    assert_eq!(s3, StatusCode::CREATED);
    let broken_id = broken_sub["id"].as_i64().unwrap();

    // Standalone practice solve for Alice on c_static.
    let (s4, standalone_sub) = send_json(
        &app,
        Method::POST,
        "/api/submissions",
        Some(alice.clone()),
        Some(json!({
            "content": "flag{static_ok}",
            "challenge_id": c_static
        })),
    )
    .await;
    assert_eq!(s4, StatusCode::CREATED);
    let standalone_id = standalone_sub["id"].as_i64().unwrap();

    let mut all_ids = vec![b_sub1_id, broken_id, standalone_id];
    all_ids.extend(&dup_ids);
    wait_until_terminal(&db, &all_ids).await;

    // Verify terminal statuses & hydrated scores via GET /api/submissions.
    let (st, list_body) = send_json(
        &app,
        Method::GET,
        &format!("/api/submissions?game_id={game_id}&size=50"),
        Some(alice.clone()),
        None,
    )
    .await;
    assert_eq!(st, StatusCode::OK);
    let subs = list_body["submissions"].as_array().unwrap();
    let find_sub = |id: i64| subs.iter().find(|s| s["id"] == id).unwrap();
    let expected_base = math::curve(1000, 100, 10, 2);
    assert_eq!(find_sub(wrong_id)["status"], "incorrect");
    assert_eq!(find_sub(wrong_id)["pts"], 0);
    assert_eq!(find_sub(a_sub1_id)["status"], "correct");
    assert_eq!(find_sub(a_sub1_id)["pts"], expected_base * 110 / 100);
    assert_eq!(find_sub(a_sub1_id)["rank"], 1);
    assert_eq!(find_sub(b_sub1_id)["status"], "correct");
    assert_eq!(find_sub(b_sub1_id)["pts"], expected_base * 105 / 100);
    assert_eq!(find_sub(b_sub1_id)["rank"], 2);
    assert_eq!(find_sub(broken_id)["status"], "failed");
    for dup_id in &dup_ids {
        assert_eq!(find_sub(*dup_id)["status"], "duplicate");
        assert_eq!(find_sub(*dup_id)["pts"], 0);
    }

    // -------------------------------------------------------------------------
    // 2. GET /api/games/{id}/scoreboard & GET /api/games/{id}/teams/us
    // -------------------------------------------------------------------------
    let (st, sb) = send_json(
        &app,
        Method::GET,
        &format!("/api/games/{game_id}/scoreboard?page=1&size=10"),
        None,
        None,
    )
    .await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(sb["total"], 3); // Alpha, Beta, Charlie (Pending excluded)
    let records = sb["records"].as_array().unwrap();
    assert_eq!(records[0]["team"]["id"], t_alpha);
    assert_eq!(records[0]["team"]["rank"], 1);
    assert_eq!(records[0]["team"]["pts"], expected_base * 110 / 100);
    assert_eq!(
        records[0]["submissions"][0]["pts"],
        expected_base * 110 / 100
    );
    assert_eq!(records[1]["team"]["id"], t_beta);
    assert_eq!(records[1]["team"]["rank"], 2);
    assert_eq!(records[1]["team"]["pts"], expected_base * 105 / 100);
    assert_eq!(records[2]["team"]["id"], t_charlie);
    assert_eq!(records[2]["team"]["rank"], 3);
    assert_eq!(records[2]["team"]["pts"], 0);
    let sb_challenges = sb["challenges"].as_array().unwrap();
    assert_eq!(sb_challenges.len(), 3);
    assert_eq!(
        sb_challenges
            .iter()
            .find(|c| c["challenge_id"] == c_static)
            .unwrap()["pts"],
        expected_base
    );

    // Self-team endpoint shows pts & rank when not blacked out.
    let (st, us_body) = send_json(
        &app,
        Method::GET,
        &format!("/api/games/{game_id}/teams/us"),
        Some(alice.clone()),
        None,
    )
    .await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(us_body["team"]["pts"], expected_base * 110 / 100);
    assert_eq!(us_body["team"]["rank"], 1);

    // -------------------------------------------------------------------------
    // 3. POST /api/challenges/status (both game scope and standalone scope)
    // -------------------------------------------------------------------------
    let (st, status_body) = send_json(
        &app,
        Method::POST,
        "/api/challenges/status",
        Some(alice.clone()),
        Some(json!({
            "challenge_ids": [c_static, c_suid, c_broken],
            "game_id": game_id,
            "team_id": t_alpha
        })),
    )
    .await;
    assert_eq!(st, StatusCode::OK);
    let c_static_status = &status_body["statuses"][&c_static.to_string()];
    assert_eq!(c_static_status["solved"], true);
    assert_eq!(c_static_status["solved_times"], 2);
    assert_eq!(c_static_status["pts"], expected_base);
    assert_eq!(c_static_status["bloods"].as_array().unwrap().len(), 2);
    assert_eq!(c_static_status["bloods"][0]["id"], a_sub1_id);
    assert_eq!(c_static_status["bloods"][1]["id"], b_sub1_id);
    assert_eq!(c_static_status["cheated"], false);

    // Standalone practice scope
    let (st, standalone_status) = send_json(
        &app,
        Method::POST,
        "/api/challenges/status",
        Some(alice.clone()),
        Some(json!({
            "challenge_ids": [c_static],
            "user_id": u_alice
        })),
    )
    .await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(
        standalone_status["statuses"][&c_static.to_string()]["solved"],
        true
    );
    assert_eq!(
        standalone_status["statuses"][&c_static.to_string()]["solved_times"],
        1
    );

    // -------------------------------------------------------------------------
    // 4. Admin Teams (sorts=rank, -rank, name filter + sorts=rank)
    // -------------------------------------------------------------------------
    let (st, admin_teams) = send_json(
        &app,
        Method::GET,
        &format!("/api/management/games/{game_id}/teams?sorts=rank&page=1&size=10"),
        Some(admin.clone()),
        None,
    )
    .await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(admin_teams["total"], 4); // 3 Passed + 1 Pending
    let rows = admin_teams["teams"].as_array().unwrap();
    assert_eq!(rows[0]["id"], t_alpha);
    assert_eq!(rows[0]["rank"], 1);
    assert_eq!(rows[1]["id"], t_beta);
    assert_eq!(rows[1]["rank"], 2);
    assert_eq!(rows[2]["id"], t_charlie);
    assert_eq!(rows[2]["rank"], 3);
    assert_eq!(rows[3]["id"], t_pending);
    assert_eq!(rows[3]["rank"], 0);

    // Filtered + rank-sorted
    let (st, filtered_teams) = send_json(
        &app,
        Method::GET,
        &format!("/api/management/games/{game_id}/teams?name=Beta&sorts=rank&page=1&size=10"),
        Some(admin.clone()),
        None,
    )
    .await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(filtered_teams["total"], 1);
    assert_eq!(filtered_teams["teams"][0]["id"], t_beta);
    assert_eq!(filtered_teams["teams"][0]["rank"], 2);

    // -------------------------------------------------------------------------
    // 5. Admin submission override -> 409 Conflict via PG partial unique index
    // -------------------------------------------------------------------------
    let (st, conflict_body) = send_json(
        &app,
        Method::PATCH,
        &format!("/api/management/submissions/{wrong_id}/status"),
        Some(admin.clone()),
        Some(json!({ "status": "correct" })),
    )
    .await;
    assert_eq!(st, StatusCode::CONFLICT);
    assert_eq!(conflict_body["code"], "correct_submission_already_exists");

    // -------------------------------------------------------------------------
    // 6. Cheat detection bans both teams & excludes banned team from scoring
    // -------------------------------------------------------------------------
    let alpha_suid_flag = format!(
        "flag{{{}}}",
        cds_checker::util::suid::encode(t_alpha, TEST_KEY, false)
    );
    let (st, cheat_sub) = send_json(
        &app,
        Method::POST,
        "/api/submissions",
        Some(charlie.clone()),
        Some(json!({
            "content": alpha_suid_flag,
            "game_id": game_id,
            "team_id": t_charlie,
            "challenge_id": c_suid
        })),
    )
    .await;
    assert_eq!(st, StatusCode::CREATED);
    let cheat_id = cheat_sub["id"].as_i64().unwrap();
    wait_until_terminal(&db, &[cheat_id]).await;

    // Scoreboard now contains ONLY Team Beta, inheriting 1st blood (1100 pts)
    // on c_static!
    let (st, sb_after_cheat) = send_json(
        &app,
        Method::GET,
        &format!("/api/games/{game_id}/scoreboard?page=1&size=10"),
        None,
        None,
    )
    .await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(sb_after_cheat["total"], 1);
    assert_eq!(sb_after_cheat["records"][0]["team"]["id"], t_beta);
    assert_eq!(sb_after_cheat["records"][0]["team"]["rank"], 1);
    assert_eq!(sb_after_cheat["records"][0]["team"]["pts"], 1100);

    // Admin force-refresh endpoint works synchronously.
    let (st, calc_body) = send_json(
        &app,
        Method::POST,
        &format!("/api/management/games/{game_id}/calculate"),
        Some(admin.clone()),
        None,
    )
    .await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(calc_body, json!({}));

    // Blackout hides pts/rank on /teams/us and forbids /scoreboard.
    db.conn
        .execute_unprepared(&format!(
            "UPDATE games SET blacked_out = true WHERE id = {game_id}"
        ))
        .await
        .unwrap();
    let (st, beta_us) = send_json(
        &app,
        Method::GET,
        &format!("/api/games/{game_id}/teams/us"),
        Some(bob.clone()),
        None,
    )
    .await;
    assert_eq!(st, StatusCode::OK);
    assert!(beta_us["team"].get("pts").is_none());
    assert!(beta_us["team"].get("rank").is_none());

    // -------------------------------------------------------------------------
    // 7. Stale processing lease self-heals via fallback poll without
    //    recovery.rs
    // -------------------------------------------------------------------------
    let stale_id: i64 = db
        .conn
        .query_one_raw(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "INSERT INTO submissions (content, status, challenge_id, user_id, team_id, game_id, created_at, processing_at, claims) VALUES ('flag{static_ok}', 'processing', $1, $2, $3, $4, now() - interval '30 seconds', now() - interval '25 seconds', 1) RETURNING id",
            [c_static.into(), u_bob.into(), t_beta.into(), game_id.into()],
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get("", "id")
        .unwrap();
    // Deliberately do NOT send NATS wakeup; fallback poll must reclaim it
    // within ~2s.
    wait_until_terminal(&db, &[stale_id]).await;
    let stale_status: String = db
        .conn
        .query_one_raw(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT status FROM submissions WHERE id = $1",
            [stale_id.into()],
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get("", "status")
        .unwrap();
    assert_eq!(stale_status, "duplicate");

    // Cleanup.
    db.conn
        .execute_unprepared(&format!(
            "DELETE FROM games WHERE id = {game_id}; DELETE FROM challenges WHERE id IN ({c_static}, {c_suid}, {c_broken}); DELETE FROM users WHERE id IN ({u_alice}, {u_bob}, {u_charlie}, {u_admin});"
        ))
        .await
        .unwrap();
    let _ = cache.clear_namespace().await;
}

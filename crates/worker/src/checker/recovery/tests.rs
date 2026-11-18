//! These tests need disposable PostgreSQL, JetStream and S3 services.
//! Set CDS_TEST_DATABASE_URL and use disposable services at the ports below.
//! The child test runs the actual checker and calculator in separate OS
//! processes, not mock consumers.

use std::{
    fs::File,
    process::{Child, Command, Stdio},
    time::Duration,
};

use cds_db::{
    DB,
    sea_orm::{ConnectOptions, ConnectionTrait, Database, DbBackend, Set, Statement},
};
use cds_queue::{
    Queue,
    async_nats::jetstream::{self, consumer::pull, stream::DiscardPolicy},
};
use time::OffsetDateTime;

use super::super::{Context, SUBJECT};

async fn dependencies() -> (DB, Queue, cds_media::Media) {
    let mut options = ConnectOptions::new(std::env::var("CDS_TEST_DATABASE_URL").unwrap());
    options
        .min_connections(1)
        .max_connections(8)
        .sqlx_logging(false);
    let db = DB {
        conn: Database::connect(options).await.unwrap(),
    };
    let mut env = cds_env::Env::default();
    env.queue.host = "127.0.0.1".to_owned();
    env.queue.port = 14222;
    env.media.endpoint = "http://127.0.0.1:19000".to_owned();
    env.media.bucket = "cdsctf-recovery-tests".to_owned();
    env.media.access_key = "recoverytest".to_owned();
    env.media.secret_key = "recoverytest123".to_owned();
    let queue = cds_queue::init(&env).await.unwrap();
    let media = cds_media::init(&env).await.unwrap();
    (db, queue, media)
}

#[tokio::test]
#[ignore = "internal child process for the disposable-service recovery test"]
async fn live_worker_process() {
    if std::env::var("CDS_RECOVERY_TEST_CHILD").as_deref() != Ok("1") {
        println!("Internal worker fixture; launched by the live recovery test");
        return;
    }
    tracing_subscriber::fmt()
        .with_env_filter("cds_worker=info,cds_worker::checker::recovery=debug")
        .init();
    let (db, queue, media) = dependencies().await;
    let checker = cds_checker::init(&media).unwrap();
    crate::calculator::spawn(&db, &queue).await;
    super::super::spawn(&db, &queue, &checker).await;
    println!("LIVE_WORKER_READY pid={}", std::process::id());
    std::future::pending::<()>().await;
}

struct Workers(Vec<Child>);

impl Drop for Workers {
    fn drop(&mut self) {
        for child in &mut self.0 {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

fn start_workers() -> Workers {
    let mut workers = Workers(Vec::new());
    std::fs::create_dir_all("target/submission-delivery-tests").unwrap();
    for index in 0..2 {
        let log = File::create(format!(
            "target/submission-delivery-tests/worker-{index}.log"
        ))
        .unwrap();
        let child = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "checker::recovery::tests::live_worker_process",
                "--ignored",
                "--nocapture",
            ])
            .env("CDS_RECOVERY_TEST_CHILD", "1")
            .stdout(Stdio::from(log.try_clone().unwrap()))
            .stderr(Stdio::from(log))
            .spawn()
            .unwrap();
        workers.0.push(child);
    }
    workers
}

async fn scalar(db: &DB, sql: &str) -> i64 {
    db.conn
        .query_one_raw(Statement::from_string(DbBackend::Postgres, sql))
        .await
        .unwrap()
        .unwrap()
        .try_get("", "value")
        .unwrap()
}

async fn batch(db: &DB, challenge: i64, flag: &str) -> Vec<i64> {
    let mut ids = Vec::new();
    for _ in 0..32 {
        let row = cds_db::submission::create_record(
            &db.conn,
            cds_db::submission::ActiveModel {
                content: Set(flag.to_owned()),
                status: Set(cds_db::submission::Status::Queued),
                challenge_id: Set(challenge),
                user_id: Set(1),
                team_id: Set(Some(1)),
                game_id: Set(Some(1)),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        ids.push(row.id);
    }
    ids
}

async fn assert_scored(db: &DB, challenge: i64) {
    tokio::time::timeout(Duration::from_secs(60), async {
        loop {
            let pending = scalar(db, &format!("SELECT count(*) AS value FROM submissions WHERE challenge_id={challenge} AND status IN ('queued','processing')")).await;
            let revision = scalar(db, "SELECT score_revision AS value FROM games WHERE id=1").await;
            let pts = scalar(db, "SELECT pts AS value FROM teams WHERE id=1").await;
            if pending == 0 && revision == 0 && pts == challenge * 1000 { break; }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }).await.expect("work did not complete and score without restarting workers");
    assert_eq!(scalar(db, &format!("SELECT count(*) AS value FROM submissions WHERE challenge_id={challenge} AND status='correct'")).await, 1);
    assert_eq!(scalar(db, &format!("SELECT count(*) AS value FROM submissions WHERE challenge_id={challenge} AND status='duplicate'")).await, 31);
}

#[tokio::test]
#[ignore = "requires disposable services on 15432 (PostgreSQL), 14222 (JetStream), 19000 (S3)"]
async fn live_recovery_survives_missing_publication_broker_rejection_and_duplicates() {
    let (db, queue, media) = dependencies().await;
    cds_migrator::run(&db).await.unwrap();
    db.conn.execute_unprepared(r#"
        INSERT INTO users (id,name,username,hashed_password,created_at,updated_at)
            VALUES (1,'Recovery test','recovery-test','unused',now(),now());
        INSERT INTO games (id,title,enabled,public,timeslots,started_at,frozen_at,ended_at,created_at)
            VALUES (1,'Recovery test',true,true,'[]',now()-interval '1 day',now()+interval '1 day',now()+interval '1 day',now());
        INSERT INTO teams (id,game_id,name,state) VALUES (1,1,'Recovery test',3);
    "#).await.unwrap();
    let key = "11".repeat(64);
    let flag = format!(
        "flag{{{}}}",
        cds_checker::util::suid::encode(1, &key, false)
    );
    let script = r#"
        function generate(subject) return {} end
        function check(subject, content)
            local flag = checker.audit.parse(content)
            if checker.suid.decode(flag.content) == subject then
                return checker.audit.correct()
            end
            return checker.audit.incorrect()
        end
    "#;
    for id in 1..=9 {
        db.conn.execute_raw(Statement::from_sql_and_values(DbBackend::Postgres,
            "INSERT INTO challenges (id,title,description,category,tags,has_instance,has_attachment,public,has_writeup,checker,created_at,updated_at) VALUES ($1,'Recovery test','',0,'{}',false,false,true,false,$2,now(),now())",
            [id.into(), script.into()])).await.unwrap();
        db.conn.execute_raw(Statement::from_sql_and_values(DbBackend::Postgres,
            "INSERT INTO game_challenges (game_id,challenge_id,max_pts,min_pts,bonus_ratios,enabled) VALUES (1,$1,1000,1000,'{}',true)",
            [id.into()])).await.unwrap();
        media
            .save(
                format!("challenges/{id}"),
                ".key".to_owned(),
                key.as_bytes().to_vec(),
            )
            .await
            .unwrap();
    }
    let checker = cds_checker::init(&media).unwrap();
    let ctx = Context::new(&db, &queue, &checker);
    let mut workers = start_workers();
    let pids: Vec<_> = workers.0.iter().map(Child::id).collect();
    // Wait until both child workers have completed their initial scan. This
    // ensures that missing-publication cases exercise the periodic scan.
    tokio::time::timeout(Duration::from_secs(15), async {
        loop {
            if (0..2).all(|i| {
                let log = std::fs::read_to_string(format!(
                    "target/submission-delivery-tests/worker-{i}.log"
                ))
                .unwrap_or_default();
                log.contains("LIVE_WORKER_READY")
                    && log.contains("queued publication scan completed")
            }) {
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .unwrap();
    let client = cds_queue::async_nats::connect("127.0.0.1:14222")
        .await
        .unwrap();
    let jet = jetstream::new(client);
    let stream_name = SUBJECT.replace('.', "-");
    let mut stream = jet.get_stream(&stream_name).await.unwrap();

    for repeat in 0..3 {
        let missing = repeat * 3 + 1;
        batch(&db, missing, &flag).await; // Deliberately do not publish.
        assert_scored(&db, missing).await;
        println!("PASS repeat={repeat} missing publication, 32 terminal, one solve");

        let rejected = missing + 1;
        let info = stream.info().await.unwrap().clone();
        let original = info.config.clone();
        let mut full = original.clone();
        full.max_messages = info.state.messages as i64;
        full.discard = DiscardPolicy::New;
        jet.update_stream(full).await.unwrap();
        batch(&db, rejected, &flag).await;
        assert!(
            super::republish_queued(&ctx, OffsetDateTime::now_utc())
                .await
                .is_err()
        );
        tokio::time::sleep(Duration::from_secs(22)).await;
        assert_eq!(scalar(&db, &format!("SELECT count(*) AS value FROM submissions WHERE challenge_id={rejected} AND status='queued'")).await, 32);
        jet.update_stream(original).await.unwrap();
        assert_scored(&db, rejected).await;
        println!("PASS repeat={repeat} broker rejection and recovery, no worker restart");

        let duplicate = missing + 2;
        let ids = batch(&db, duplicate, &flag).await;
        for id in &ids {
            for _ in 0..3 {
                queue.publish(SUBJECT, id, None).await.unwrap();
            }
        }
        assert_scored(&db, duplicate).await;
        let claims = scalar(&db, &format!("SELECT sum(claims)::bigint AS value FROM submissions WHERE challenge_id={duplicate}")).await;
        for id in ids {
            queue.publish(SUBJECT, id, None).await.unwrap();
        }
        let mut consumer = stream.get_consumer::<pull::Config>("worker").await.unwrap();
        tokio::time::timeout(Duration::from_secs(30), async {
            loop {
                let info = consumer.info().await.unwrap();
                if info.num_pending == 0 && info.num_ack_pending == 0 {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        })
        .await
        .unwrap();
        assert_eq!(scalar(&db, &format!("SELECT sum(claims)::bigint AS value FROM submissions WHERE challenge_id={duplicate}")).await, claims);
        assert_scored(&db, duplicate).await;
        println!("PASS repeat={repeat} duplicate delivery and terminal redelivery");
        assert_eq!(workers.0.iter().map(Child::id).collect::<Vec<_>>(), pids);
        assert!(
            workers
                .0
                .iter_mut()
                .all(|child| child.try_wait().unwrap().is_none())
        );
    }
    println!(
        "PASS 288 valid SUID submissions; 9 correct, 279 duplicate, 9000 points, revision zero; two worker PIDs unchanged"
    );
}

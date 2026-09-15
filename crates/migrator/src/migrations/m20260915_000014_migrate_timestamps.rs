//! Convert Unix-second timestamp columns to PostgreSQL timestamptz values.

use async_trait::async_trait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260915_000014_migrate_timestamps"
    }
}

const TIMESTAMP_COLUMNS: &[(&str, &[&str])] = &[
    ("users", &["deleted_at", "created_at", "updated_at"]),
    (
        "games",
        &["started_at", "frozen_at", "ended_at", "created_at"],
    ),
    ("challenges", &["deleted_at", "created_at", "updated_at"]),
    ("game_notices", &["created_at"]),
    ("game_challenges", &["frozen_at"]),
    (
        "submissions",
        &["created_at", "processing_at", "checked_at"],
    ),
    ("notes", &["created_at", "updated_at"]),
    ("idps", &["created_at", "updated_at"]),
    ("user_idps", &["created_at", "updated_at"]),
];

fn alter_sql(table: &str, column: &str, to_timestamptz: bool) -> String {
    if to_timestamptz {
        format!(
            "ALTER TABLE \"{table}\" ALTER COLUMN \"{column}\" TYPE TIMESTAMPTZ USING to_timestamp(\"{column}\")"
        )
    } else {
        format!(
            "ALTER TABLE \"{table}\" ALTER COLUMN \"{column}\" TYPE BIGINT USING floor(extract(epoch FROM \"{column}\"))::BIGINT"
        )
    }
}

async fn execute_timestamp_alters(
    manager: &SchemaManager<'_>,
    to_timestamptz: bool,
) -> Result<(), DbErr> {
    let db = manager.get_connection();
    for (table, columns) in TIMESTAMP_COLUMNS {
        for column in *columns {
            db.execute_raw(Statement::from_string(
                manager.get_database_backend(),
                alter_sql(table, column, to_timestamptz),
            ))
            .await?;
        }
    }
    Ok(())
}

async fn migrate_timeslots(manager: &SchemaManager<'_>, to_timestamptz: bool) -> Result<(), DbErr> {
    let db = manager.get_connection();
    let sql = if to_timestamptz {
        r#"
        UPDATE "games"
        SET "timeslots" = COALESCE((
            SELECT jsonb_agg(
                jsonb_set(
                    jsonb_set(
                        slot,
                        '{started_at}',
                        CASE
                            WHEN jsonb_typeof(slot->'started_at') = 'number' THEN
                                to_jsonb(to_char(
                                    to_timestamp((slot->>'started_at')::double precision)
                                        AT TIME ZONE 'UTC',
                                    'YYYY-MM-DD"T"HH24:MI:SS.US"Z"'
                                ))
                            ELSE slot->'started_at'
                        END
                    ),
                    '{ended_at}',
                    CASE
                        WHEN jsonb_typeof(slot->'ended_at') = 'number' THEN
                            to_jsonb(to_char(
                                to_timestamp((slot->>'ended_at')::double precision)
                                    AT TIME ZONE 'UTC',
                                'YYYY-MM-DD"T"HH24:MI:SS.US"Z"'
                            ))
                        ELSE slot->'ended_at'
                    END
                ) ORDER BY ordinal
            )
            FROM jsonb_array_elements("timeslots") WITH ORDINALITY AS entries(slot, ordinal)
        ), '[]'::jsonb)
        "#
    } else {
        r#"
        UPDATE "games"
        SET "timeslots" = COALESCE((
            SELECT jsonb_agg(
                jsonb_set(
                    jsonb_set(
                        slot,
                        '{started_at}',
                        to_jsonb(floor(extract(epoch FROM (slot->>'started_at')::timestamptz))::bigint)
                    ),
                    '{ended_at}',
                    to_jsonb(floor(extract(epoch FROM (slot->>'ended_at')::timestamptz))::bigint)
                ) ORDER BY ordinal
            )
            FROM jsonb_array_elements("timeslots") WITH ORDINALITY AS entries(slot, ordinal)
        ), '[]'::jsonb)
        "#
    };
    db.execute_raw(Statement::from_string(
        manager.get_database_backend(),
        sql.to_owned(),
    ))
    .await?;
    Ok(())
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        migrate_timeslots(manager, true).await?;
        execute_timestamp_alters(manager, true).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        execute_timestamp_alters(manager, false).await?;
        migrate_timeslots(manager, false).await
    }
}

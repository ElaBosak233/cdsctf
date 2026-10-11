//! Replaces the legacy management access flag with resource permissions.

use async_trait::async_trait;
use sea_orm::ConnectionTrait;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261011_000015_expand_management_permissions"
    }
}

const MANAGEMENT_PERMISSIONS: &str = "ARRAY[
    'management:config', 'management:challenge', 'management:game',
    'management:idp', 'management:instance', 'management:submission',
    'management:user'
]::TEXT[]";

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"
                CREATE OR REPLACE FUNCTION cds_jsonb_text_array(value JSONB)
                RETURNS TEXT[]
                LANGUAGE SQL
                IMMUTABLE
                AS $$ SELECT COALESCE(array_agg(item), ARRAY[]::TEXT[])
                      FROM jsonb_array_elements_text(value) AS item $$;
            "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
                DO $$
                BEGIN
                    IF EXISTS (
                        SELECT 1
                        FROM information_schema.columns
                        WHERE table_schema = current_schema()
                          AND table_name = 'users'
                          AND column_name = 'permissions'
                          AND udt_name = 'jsonb'
                    ) THEN
                        ALTER TABLE "users"
                            ALTER COLUMN "permissions" DROP DEFAULT,
                            ALTER COLUMN "permissions" TYPE TEXT[]
                            USING cds_jsonb_text_array("permissions"),
                            ALTER COLUMN "permissions" SET DEFAULT ARRAY[]::TEXT[];
                    END IF;
                END $$;
            "#,
        )
        .await?;
        db.execute_unprepared("DROP FUNCTION cds_jsonb_text_array(JSONB);")
            .await?;
        db.execute_unprepared(&format!(
            r#"
                UPDATE "users"
                SET "permissions" = array_cat(
                    array_remove("permissions", 'management:access'),
                    {MANAGEMENT_PERMISSIONS}
                )
                WHERE 'management:access' = ANY("permissions");
            "#
        ))
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(&format!(
            r#"
                UPDATE "users"
                SET "permissions" = array_cat(
                    ARRAY(
                        SELECT permission
                        FROM unnest("permissions") AS permission
                        WHERE permission NOT IN (
                            'management:config', 'management:challenge', 'management:game',
                            'management:idp', 'management:instance', 'management:submission',
                            'management:user'
                        )
                    )::TEXT[],
                    ARRAY['management:access']::TEXT[]
                )
                WHERE "permissions" @> {MANAGEMENT_PERMISSIONS};
            "#
        ))
        .await?;
        db.execute_unprepared(
            r#"
                ALTER TABLE "users"
                    ALTER COLUMN "permissions" DROP DEFAULT,
                    ALTER COLUMN "permissions" TYPE JSONB
                    USING to_jsonb("permissions"),
                    ALTER COLUMN "permissions" SET DEFAULT '[]'::jsonb;
            "#,
        )
        .await?;
        Ok(())
    }
}

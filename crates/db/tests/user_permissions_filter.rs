use cds_db::{Permission, sea_orm, user};
use sea_orm::{ConnectionTrait, Database, TransactionTrait};
use user::PermissionFilter;

#[tokio::test]
#[ignore = "requires CDS_TEST_DATABASE_URL pointing to PostgreSQL"]
async fn user_permissions_filter_applies_before_counting_and_pagination() {
    let database = Database::connect(std::env::var("CDS_TEST_DATABASE_URL").unwrap())
        .await
        .unwrap();
    let transaction = database.begin().await.unwrap();
    transaction
        .execute_unprepared(
            r#"
            CREATE TEMP TABLE users (
                id BIGINT PRIMARY KEY,
                name TEXT NOT NULL,
                username TEXT NOT NULL,
                description TEXT,
                banned BOOLEAN NOT NULL DEFAULT FALSE,
                permissions TEXT[] NOT NULL,
                hashed_password TEXT NOT NULL DEFAULT '',
                avatar_hash TEXT,
                deleted_at TIMESTAMPTZ,
                created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
            ) ON COMMIT DROP;
            CREATE TEMP TABLE emails (
                user_id BIGINT NOT NULL,
                verified BOOLEAN NOT NULL
            ) ON COMMIT DROP;

            INSERT INTO users (id, name, username, banned, permissions) VALUES
                (1, 'Test Basic', 'basic', false, ARRAY['basic']),
                (2, 'Test Config', 'config', false, ARRAY['basic', 'management:config']),
                (3, 'Game Manager', 'Test_manager', false, ARRAY['management:game']),
                (4, 'Test Banned', 'banned', true, ARRAY['management:config']),
                (5, 'Outside Search', 'outside', false, ARRAY['management:game']),
                (6, 'Test Deleted', 'deleted', false, ARRAY['management:config']),
                (7, 'No permissions', 'percent%_value', false, ARRAY[]::TEXT[]),
                (8, 'Other match', 'percent12value', false, ARRAY['management:game']);
            UPDATE users SET deleted_at = now() WHERE id = 6;
            "#,
        )
        .await
        .unwrap();

    let options = user::FindUserOptions {
        keyword: Some("test".into()),
        banned: Some(false),
        permissions: Some(PermissionFilter::AnyOf(vec![
            Permission::ManagementConfig,
            Permission::ManagementGame,
        ])),
        sorts: Some("id".into()),
        page: Some(2),
        size: Some(1),
        ..Default::default()
    };
    let (rows, total) = user::find::<user::UserAccountView>(&transaction, options.clone())
        .await
        .unwrap();
    assert_eq!(total, 2);
    assert_eq!(rows.iter().map(|row| row.id).collect::<Vec<_>>(), [3]);

    let (rows, total) = user::find::<user::UserAccountView>(
        &transaction,
        user::FindUserOptions {
            permissions: None,
            page: Some(1),
            ..options.clone()
        },
    )
    .await
    .unwrap();
    assert_eq!(total, 3);
    assert_eq!(rows.iter().map(|row| row.id).collect::<Vec<_>>(), [1]);

    let (rows, total) = user::find::<user::UserAccountView>(
        &transaction,
        user::FindUserOptions {
            permissions: Some(PermissionFilter::AnyOf(vec![Permission::ManagementGame])),
            page: Some(1),
            ..options.clone()
        },
    )
    .await
    .unwrap();
    assert_eq!(total, 1);
    assert_eq!(rows.iter().map(|row| row.id).collect::<Vec<_>>(), [3]);

    let (rows, total) = user::find::<user::UserAccountView>(
        &transaction,
        user::FindUserOptions {
            permissions: Some(PermissionFilter::NoneOf(vec![
                Permission::ManagementConfig,
                Permission::ManagementGame,
            ])),
            keyword: None,
            page: Some(1),
            size: Some(10),
            ..options
        },
    )
    .await
    .unwrap();
    assert_eq!(total, 2);
    assert_eq!(rows.iter().map(|row| row.id).collect::<Vec<_>>(), [1, 7]);

    let (rows, total) = user::find::<user::UserAccountView>(
        &transaction,
        user::FindUserOptions {
            keyword: Some("%_".into()),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert_eq!(total, 1);
    assert_eq!(rows.iter().map(|row| row.id).collect::<Vec<_>>(), [7]);

    transaction.rollback().await.unwrap();
}

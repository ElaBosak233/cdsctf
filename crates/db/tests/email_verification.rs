use cds_db::{DbError, Permission, UserAccountView, email, sea_orm, user};
use sea_orm::{ConnectOptions, ConnectionTrait, Database, Set, TransactionTrait, Unchanged};

#[tokio::test]
#[ignore = "requires CDS_TEST_DATABASE_URL pointing to PostgreSQL"]
async fn email_verification_grants_basic_once_and_serializes_address_changes() {
    let database_url = std::env::var("CDS_TEST_DATABASE_URL").unwrap();
    let database = Database::connect(&database_url).await.unwrap();
    let schema = format!("email_verification_test_{}", std::process::id());
    database
        .execute_unprepared(&format!("CREATE SCHEMA {schema}"))
        .await
        .unwrap();
    let mut options = ConnectOptions::new(database_url);
    options.set_schema_search_path(&schema).max_connections(4);
    let conn = Database::connect(options).await.unwrap();
    conn.execute_unprepared(
        r#"
        CREATE TABLE users (
            id BIGSERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            username VARCHAR UNIQUE NOT NULL,
            description TEXT,
            banned BOOLEAN NOT NULL DEFAULT FALSE,
            permissions TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[],
            hashed_password VARCHAR NOT NULL,
            avatar_hash VARCHAR,
            deleted_at TIMESTAMPTZ,
            created_at TIMESTAMPTZ NOT NULL,
            updated_at TIMESTAMPTZ NOT NULL
        );
        CREATE TABLE emails (
            email VARCHAR PRIMARY KEY,
            user_id BIGINT NOT NULL REFERENCES users(id),
            verified BOOLEAN NOT NULL DEFAULT FALSE
        );
        "#,
    )
    .await
    .unwrap();

    let register = |username: &'static str| {
        let conn = &conn;
        async move {
            let transaction = conn.begin().await.unwrap();
            let is_first = user::is_first_registration(&transaction).await.unwrap();
            let permissions = if is_first {
                vec![Permission::ManagementConfig]
            } else {
                Vec::new()
            };
            let account = user::create::<UserAccountView>(
                &transaction,
                user::ActiveModel {
                    name: Set(username.into()),
                    username: Set(username.into()),
                    hashed_password: Set("old-password".to_owned()),
                    permissions: Set(permissions),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
            email::create(
                &transaction,
                account.id,
                format!("{username}@example.test"),
                false,
            )
            .await
            .unwrap();
            transaction.commit().await.unwrap();
            (is_first, account)
        }
    };
    let ((first, a), (second, b)) = tokio::join!(register("first"), register("second"));
    assert_ne!(first, second);
    let (account, regular) = if first { (a, b) } else { (b, a) };
    assert!(regular.permissions.is_empty());
    let bound_address = "bound@example.test";
    email::create(&conn, regular.id, bound_address.to_owned(), true)
        .await
        .unwrap();
    assert_eq!(
        user::find_by_verified_email(&conn, bound_address)
            .await
            .unwrap()
            .unwrap()
            .permissions,
        [Permission::Basic]
    );
    assert!(matches!(
        email::create(&conn, account.id, bound_address.to_owned(), true).await,
        Err(DbError::Conflict(code)) if code == "email_already_exists"
    ));
    assert_eq!(
        user::find_by_id::<UserAccountView>(&conn, account.id)
            .await
            .unwrap()
            .unwrap()
            .permissions,
        [Permission::ManagementConfig]
    );

    let address = format!("{}@example.test", account.username);

    assert_eq!(account.permissions, [Permission::ManagementConfig]);
    assert!(
        user::find_by_verified_email(&conn, &address)
            .await
            .unwrap()
            .is_none()
    );
    assert!(
        user::find_by_account::<UserAccountView>(&conn, address.clone())
            .await
            .unwrap()
            .is_none()
    );
    assert!(matches!(
        user::reset_password_by_verified_email(&conn, account.id, &address, "rejected".into()).await,
        Err(DbError::Conflict(code)) if code == "email_not_verified"
    ));

    email::set_verified(&conn, account.id, &address, true)
        .await
        .unwrap();
    email::set_verified(&conn, account.id, &address, true)
        .await
        .unwrap();
    let verified_user = user::find_by_verified_email(&conn, &address)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        verified_user.permissions,
        [Permission::ManagementConfig, Permission::Basic]
    );
    assert_eq!(verified_user.hashed_password, "old-password");
    user::reset_password_by_verified_email(&conn, account.id, &address, "new-password".into())
        .await
        .unwrap();
    assert_eq!(
        user::find_by_verified_email(&conn, &address)
            .await
            .unwrap()
            .unwrap()
            .hashed_password,
        "new-password"
    );

    let pending = "pending@example.test".to_owned();
    email::create(&conn, account.id, pending.clone(), false)
        .await
        .unwrap();
    assert!(matches!(
        email::delete(&conn, account.id, address.clone()).await,
        Err(DbError::Conflict(code)) if code == "last_verified_email"
    ));
    email::delete(&conn, account.id, pending).await.unwrap();

    user::update::<UserAccountView>(
        &conn,
        user::ActiveModel {
            id: Unchanged(account.id),
            permissions: Set(vec![Permission::ManagementConfig]),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let replacement = "replacement@example.test".to_owned();
    email::create(&conn, account.id, replacement.clone(), true)
        .await
        .unwrap();
    assert_eq!(
        user::find_by_id::<UserAccountView>(&conn, account.id)
            .await
            .unwrap()
            .unwrap()
            .permissions,
        [Permission::ManagementConfig]
    );

    let (deleted, unverified) = tokio::join!(
        email::delete(&conn, account.id, address.clone()),
        email::set_verified(&conn, account.id, &replacement, false),
    );
    assert_ne!(deleted.is_ok(), unverified.is_ok());
    let conflict = deleted.err().or_else(|| unverified.err()).unwrap();
    assert!(matches!(conflict, DbError::Conflict(code) if code == "last_verified_email"));
    assert_eq!(
        email::find_by_user_id::<email::EmailView>(&conn, account.id)
            .await
            .unwrap()
            .iter()
            .filter(|email| email.verified)
            .count(),
        1
    );

    conn.close().await.unwrap();
    database
        .execute_unprepared(&format!("DROP SCHEMA {schema} CASCADE"))
        .await
        .unwrap();
}

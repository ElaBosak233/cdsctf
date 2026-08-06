//! Database access for `note` — SeaORM queries, updates, and DTOs.

use std::str::FromStr;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityLoaderTrait, EntityTrait, Order,
    QueryFilter, QueryOrder,
};
use tracing::info;

pub(crate) use crate::entity::note::{Column, Entity};
use crate::traits::DbError;
pub use crate::{dto::note::NoteView, entity::note::ActiveModel};

impl TryFrom<crate::entity::note::ModelEx> for NoteView {
    type Error = DbError;

    fn try_from(note: crate::entity::note::ModelEx) -> Result<Self, Self::Error> {
        let user = note.user.as_ref().ok_or_else(|| {
            DbError::Other(anyhow::anyhow!(
                "note {} was loaded without its user relation",
                note.id
            ))
        })?;
        let challenge = note.challenge.as_ref().ok_or_else(|| {
            DbError::Other(anyhow::anyhow!(
                "note {} was loaded without its challenge relation",
                note.id
            ))
        })?;

        Ok(Self {
            id: note.id,
            content: note.content,
            public: note.public,
            user_id: note.user_id,
            user_name: user.name.clone(),
            user_avatar_hash: user.avatar_hash.clone(),
            challenge_id: note.challenge_id,
            challenge_title: challenge.title.clone(),
            challenge_category: challenge.category,
            created_at: note.created_at,
            updated_at: note.updated_at,
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct FindNotesOptions {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub public: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

/// Queries rows using filter options and returns `(rows, total_count)`.
pub async fn find(
    conn: &impl ConnectionTrait,
    FindNotesOptions {
        id,
        user_id,
        challenge_id,
        public,
        page,
        size,
        sorts,
    }: FindNotesOptions,
) -> Result<(Vec<NoteView>, u64), DbError> {
    let mut loader = Entity::load()
        .with(crate::entity::user::Entity)
        .with(crate::entity::challenge::Entity);

    if let Some(id) = id {
        loader = loader.filter(Column::Id.eq(id));
    }

    if let Some(user_id) = user_id {
        loader = loader.filter(Column::UserId.eq(user_id));
    }

    if let Some(challenge_id) = challenge_id {
        loader = loader.filter(Column::ChallengeId.eq(challenge_id));
    }

    if let Some(public) = public {
        loader = loader.filter(Column::Public.eq(public));
    }

    if let Some(sorts) = sorts {
        let sorts = sorts.split(",").collect::<Vec<&str>>();
        for sort in sorts {
            let col = match Column::from_str(sort.replace("-", "").as_str()) {
                Ok(col) => col,
                Err(_) => continue,
            };
            if sort.starts_with("-") {
                loader = loader.order_by(col, Order::Desc);
            } else {
                loader = loader.order_by(col, Order::Asc);
            }
        }
    }

    let (models, total) = match (page, size) {
        (Some(_), Some(0)) => {
            let total = loader.clone().paginate(conn, 1).num_items().await?;
            (Vec::new(), total)
        }
        (Some(page), Some(size)) => {
            let paginator = loader.paginate(conn, size);
            let total = paginator.num_items().await?;
            let models = paginator.fetch_page(page.saturating_sub(1)).await?;
            (models, total)
        }
        _ => {
            let models = loader.all(conn).await?;
            let total = models.len() as u64;
            (models, total)
        }
    };

    let notes = models
        .into_iter()
        .map(NoteView::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    Ok((notes, total))
}

/// Looks up by id.

pub async fn find_by_id(
    conn: &impl ConnectionTrait,
    note_id: i64,
) -> Result<Option<NoteView>, DbError> {
    Ok(Entity::load()
        .with(crate::entity::user::Entity)
        .with(crate::entity::challenge::Entity)
        .filter(Column::Id.eq(note_id))
        .one(conn)
        .await?
        .map(NoteView::try_from)
        .transpose()?)
}

/// Looks up by user id and challenge id.

pub async fn find_by_user_id_and_challenge_id(
    conn: &impl ConnectionTrait,
    user_id: i64,
    challenge_id: i64,
) -> Result<Option<NoteView>, DbError> {
    Ok(Entity::load()
        .with(crate::entity::user::Entity)
        .with(crate::entity::challenge::Entity)
        .filter(Column::UserId.eq(user_id))
        .filter(Column::ChallengeId.eq(challenge_id))
        .one(conn)
        .await?
        .map(NoteView::try_from)
        .transpose()?)
}

/// Inserts a new row and returns the persisted model.
pub async fn create(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<NoteView, DbError> {
    let note = model.insert(conn).await?;
    info!(
        note_id = note.id,
        user_id = note.user_id,
        challenge_id = note.challenge_id,
        public = note.public,
        "note created"
    );

    Ok(find_by_id(conn, note.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("note_{}", note.id)))?)
}

/// Applies an active model update to the database.
pub async fn update(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<NoteView, DbError> {
    let note = model.update(conn).await?;
    info!(
        note_id = note.id,
        user_id = note.user_id,
        challenge_id = note.challenge_id,
        public = note.public,
        "note updated"
    );

    Ok(find_by_id(conn, note.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("note_{}", note.id)))?)
}

/// Deletes rows matching the provided identifier or filter.
pub async fn delete(conn: &impl ConnectionTrait, note_id: i64) -> Result<(), DbError> {
    Entity::delete_by_id(note_id).exec(conn).await?;
    info!(note_id, "note deleted");

    Ok(())
}

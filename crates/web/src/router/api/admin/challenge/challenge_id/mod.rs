mod attachment;

use axum::{Router, http::StatusCode};
use cds_checker::traits::CheckerError;
use cds_db::{entity::user::Group, get_db, transfer::Challenge};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{Set, Unchanged},
    ColumnTrait, EntityTrait, NotSet, QueryFilter,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    extract::{Extension, Path, VJson},
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::put(update_challenge))
        .route("/", axum::routing::delete(delete_challenge))
        .route("/env", axum::routing::put(update_challenge_env))
        .route("/checker", axum::routing::put(update_challenge_checker))
        .nest("/attachment", attachment::router())
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateChallengeRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub is_public: Option<bool>,
    pub is_dynamic: Option<bool>,
    pub has_attachment: Option<bool>,
}

pub async fn update_challenge(
    Extension(ext): Extension<Ext>, Path(challenge_id): Path<uuid::Uuid>,
    VJson(mut body): VJson<UpdateChallengeRequest>,
) -> Result<WebResponse<Challenge>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let challenge = cds_db::entity::challenge::Entity::find_by_id(challenge_id)
        .filter(cds_db::entity::challenge::Column::DeletedAt.is_null())
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("challenge_not_found")))?;

    let challenge = cds_db::entity::challenge::ActiveModel {
        id: Unchanged(challenge.id),
        title: body.title.map_or(NotSet, Set),
        description: body.description.map_or(NotSet, Set),
        tags: body.tags.map_or(NotSet, Set),
        category: body.category.map_or(NotSet, Set),
        is_public: body.is_public.map_or(NotSet, Set),
        is_dynamic: body.is_dynamic.map_or(NotSet, Set),
        has_attachment: body.has_attachment.map_or(NotSet, Set),
        ..Default::default()
    }
    .update(get_db())
    .await?;
    let challenge = cds_db::transfer::Challenge::from(challenge);

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(challenge),
        ..Default::default()
    })
}

pub async fn delete_challenge(
    Extension(ext): Extension<Ext>, Path(challenge_id): Path<uuid::Uuid>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let challenge = cds_db::entity::challenge::Entity::find_by_id(challenge_id)
        .filter(cds_db::entity::challenge::Column::DeletedAt.is_null())
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("challenge_not_found")))?;

    let _ = cds_db::entity::challenge::ActiveModel {
        id: Set(challenge.id),
        deleted_at: Set(Some(chrono::Utc::now().timestamp())),
        ..Default::default()
    }
    .update(get_db())
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateChallengeEnvRequest {
    pub env: Option<cds_db::entity::challenge::Env>,
}

pub async fn update_challenge_env(
    Extension(ext): Extension<Ext>, Path(challenge_id): Path<uuid::Uuid>,
    VJson(body): VJson<UpdateChallengeEnvRequest>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let _ = cds_db::entity::challenge::Entity::find_by_id(challenge_id)
        .filter(cds_db::entity::challenge::Column::DeletedAt.is_null())
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("challenge_not_found")))?;

    let _ = cds_db::entity::challenge::ActiveModel {
        id: Unchanged(challenge_id),
        env: body.env.map_or(NotSet, |v| Set(Some(v))),
        ..Default::default()
    }
    .update(get_db())
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateChallengeCheckerRequest {
    pub checker: Option<String>,
}

pub async fn update_challenge_checker(
    Extension(ext): Extension<Ext>, Path(challenge_id): Path<uuid::Uuid>,
    VJson(body): VJson<UpdateChallengeCheckerRequest>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let _ = cds_db::entity::challenge::Entity::find_by_id(challenge_id)
        .filter(cds_db::entity::challenge::Column::DeletedAt.is_null())
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("challenge_not_found")))?;

    let challenge = cds_db::entity::challenge::ActiveModel {
        id: Unchanged(challenge_id),
        checker: body.checker.map_or(NotSet, |v| Set(Some(v))),
        ..Default::default()
    }
    .update(get_db())
    .await?;

    let script = challenge
        .checker
        .ok_or(WebError::BadRequest(json!("null_checker_script")))?;

    let lint = cds_checker::lint(&script);
    let msg = if let Err(lint) = lint {
        match lint {
            CheckerError::CompileError(diagnostics) => Some(diagnostics),
            err => Some(err.to_string()),
        }
    } else {
        None
    };

    Ok(WebResponse {
        code: StatusCode::OK,
        msg: msg.map(|msg| json!(msg)),
        ..Default::default()
    })
}

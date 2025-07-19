use cds_db::sea_orm::PaginatorTrait;
mod email;
mod logo;

use axum::Router;
use cds_db::{
    entity::submission::Status,
    sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set, Unchanged},
};
use serde::{Deserialize, Serialize};

use crate::{
    extract::Json,
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_config))
        .route("/", axum::routing::put(update_config))
        .nest("/logo", logo::router())
        .nest("/email", email::router())
        .route("/statistics", axum::routing::get(get_statistics))
}

pub async fn get_config() -> Result<WebResponse<cds_db::entity::config::Model>, WebError> {
    Ok(WebResponse {
        data: Some(cds_db::get_config().await),
        ..Default::default()
    })
}

pub async fn update_config(
    Json(body): Json<cds_db::entity::config::Model>,
) -> Result<WebResponse<cds_db::entity::config::Model>, WebError> {
    let config = cds_db::entity::config::Entity::update(cds_db::entity::config::ActiveModel {
        id: Unchanged(1),
        meta: Set(body.meta),
        auth: Set(body.auth),
        email: Set(body.email),
        captcha: Set(body.captcha),
    })
    .exec(cds_db::get_db())
    .await?;

    Ok(WebResponse {
        data: Some(config),
        ..Default::default()
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Statistics {
    pub users: u64,
    pub games: u64,
    pub challenges: ChallengeStatistics,
    pub submissions: SubmissionStatistics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeStatistics {
    pub total: u64,
    pub in_game: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmissionStatistics {
    pub total: u64,
    pub solved: u64,
}

pub async fn get_statistics() -> Result<WebResponse<Statistics>, WebError> {
    Ok(WebResponse {
        data: Some(Statistics {
            users: cds_db::entity::user::Entity::find()
                .filter(cds_db::entity::user::Column::DeletedAt.is_null())
                .count(cds_db::get_db())
                .await?,
            games: cds_db::entity::game::Entity::find()
                .count(cds_db::get_db())
                .await?,
            challenges: ChallengeStatistics {
                total: cds_db::entity::challenge::Entity::find()
                    .count(cds_db::get_db())
                    .await?,
                in_game: cds_db::entity::game_challenge::Entity::find()
                    .count(cds_db::get_db())
                    .await?,
            },
            submissions: SubmissionStatistics {
                total: cds_db::entity::submission::Entity::find()
                    .count(cds_db::get_db())
                    .await?,
                solved: cds_db::entity::submission::Entity::find()
                    .filter(cds_db::entity::submission::Column::Status.eq(Status::Correct))
                    .count(cds_db::get_db())
                    .await?,
            },
        }),
        ..Default::default()
    })
}

mod email;
mod logo;

use axum::Router;
use cds_db::{
    ChallengeMini, GameChallengeMini, GameMini, Submission, UserMini,
    challenge::FindChallengeOptions,
    game::FindGameOptions,
    game_challenge::FindGameChallengeOptions,
    sea_orm::{EntityTrait, Set, Unchanged},
    submission::{FindSubmissionsOptions, Status},
    user::FindUserOptions,
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
            users: cds_db::user::find::<UserMini>(FindUserOptions {
                page: Some(1),
                size: Some(1),
                ..Default::default()
            })
            .await?
            .1,
            games: cds_db::game::find::<GameMini>(FindGameOptions {
                page: Some(1),
                size: Some(1),
                ..Default::default()
            })
            .await?
            .1,
            challenges: ChallengeStatistics {
                total: cds_db::challenge::find::<ChallengeMini>(FindChallengeOptions {
                    page: Some(1),
                    size: Some(1),
                    ..Default::default()
                })
                .await?
                .1,
                in_game: cds_db::game_challenge::find::<GameChallengeMini>(
                    FindGameChallengeOptions::default(),
                )
                .await?
                .1,
            },
            submissions: SubmissionStatistics {
                total: cds_db::submission::find::<Submission>(FindSubmissionsOptions::default())
                    .await?
                    .1,
                solved: cds_db::submission::find::<Submission>(FindSubmissionsOptions {
                    status: Some(Status::Correct),
                    page: Some(1),
                    size: Some(1),
                    ..Default::default()
                })
                .await?
                .1,
            },
        }),
        ..Default::default()
    })
}

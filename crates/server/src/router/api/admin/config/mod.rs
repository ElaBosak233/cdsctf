mod email;
mod logo;

use std::sync::Arc;

use axum::{Router, extract::State};
use cds_db::sea_orm::Set;
use serde::{Deserialize, Serialize};

use crate::{
    extract::Json,
    traits::{AppState, WebError, WebResponse},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::get(get_config))
        .route("/", axum::routing::put(update_config))
        .nest("/logo", logo::router())
        .nest("/mailbox", email::router())
        .route("/statistics", axum::routing::get(get_statistics))
}

pub async fn get_config(
    State(s): State<Arc<AppState>>,
) -> Result<WebResponse<cds_db::config::Model>, WebError> {
    Ok(WebResponse {
        data: Some(cds_db::get_config(&s.db.conn).await),
        ..Default::default()
    })
}

pub async fn update_config(
    State(s): State<Arc<AppState>>,

    Json(body): Json<cds_db::config::Model>,
) -> Result<WebResponse<cds_db::config::Model>, WebError> {
    let config = cds_db::config::save(
        &s.db.conn,
        cds_db::config::ActiveModel {
            meta: Set(body.meta),
            auth: Set(body.auth),
            email: Set(body.email),
            captcha: Set(body.captcha),
            ..Default::default()
        },
    )
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

pub async fn get_statistics(
    State(s): State<Arc<AppState>>,
) -> Result<WebResponse<Statistics>, WebError> {
    Ok(WebResponse {
        data: Some(Statistics {
            users: cds_db::user::count(&s.db.conn).await?,
            games: cds_db::game::count(&s.db.conn).await?,
            challenges: ChallengeStatistics {
                total: cds_db::challenge::count(&s.db.conn).await?,
                in_game: cds_db::game_challenge::count(&s.db.conn).await?,
            },
            submissions: SubmissionStatistics {
                total: cds_db::submission::count(&s.db.conn).await?,
                solved: cds_db::submission::count_correct(&s.db.conn).await?,
            },
        }),
        ..Default::default()
    })
}

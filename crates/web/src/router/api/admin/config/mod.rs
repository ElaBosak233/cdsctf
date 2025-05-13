mod logo;

use axum::Router;
use cds_db::{
    get_db,
    sea_orm::{EntityTrait, Set, Unchanged},
};

use crate::{
    extract::Json,
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_config))
        .route("/", axum::routing::put(update_config))
        .nest("/logo", logo::router())
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
    .exec(get_db())
    .await?;

    Ok(WebResponse {
        data: Some(config),
        ..Default::default()
    })
}

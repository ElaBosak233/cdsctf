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
    Router::new().nest("/attachment", attachment::router())
}

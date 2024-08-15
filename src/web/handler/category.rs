use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, EntityTrait, Set};

use crate::web::model::category::*;
use crate::web::traits::WebError;
use crate::{database::get_db, web::traits::Ext};
use crate::{model::user::group::Group, util::validate};

pub async fn get(Query(params): Query<GetRequest>) -> Result<impl IntoResponse, WebError> {
    let (categories, total) = crate::model::category::find(params.id, params.name).await?;

    return Ok((
        StatusCode::OK,
        Json(GetResponse {
            code: StatusCode::OK.as_u16(),
            data: categories,
            total: total,
        }),
    ));
}

pub async fn create(
    Extension(ext): Extension<Ext>, validate::Json(body): validate::Json<CreateRequest>,
) -> Result<impl IntoResponse, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(String::from("")));
    }
    let category = crate::model::category::ActiveModel {
        name: Set(body.name),
        color: Set(body.color),
        icon: Set(body.icon),
        ..Default::default()
    }
    .insert(&get_db())
    .await?;

    return Ok((
        StatusCode::OK,
        Json(CreateResponse {
            code: StatusCode::OK.as_u16(),
            data: category,
        }),
    ));
}

pub async fn update(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
    validate::Json(mut body): validate::Json<UpdateRequest>,
) -> Result<impl IntoResponse, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(String::from("")));
    }

    body.id = Some(id);
    let category = crate::model::category::ActiveModel {
        id: body.id.map_or(NotSet, |v| Set(v)),
        name: body.name.map_or(NotSet, |v| Set(v)),
        color: body.color.map_or(NotSet, |v| Set(v)),
        icon: body.icon.map_or(NotSet, |v| Set(v)),
        ..Default::default()
    }
    .update(&get_db())
    .await?;

    return Ok((
        StatusCode::OK,
        Json(UpdateResponse {
            code: StatusCode::OK.as_u16(),
            data: category,
        }),
    ));
}

pub async fn delete(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<impl IntoResponse, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(String::from("")));
    }

    let _ = crate::model::category::Entity::delete_by_id(id)
        .exec(&get_db())
        .await?;

    return Ok((
        StatusCode::OK,
        Json(DeleteResponse {
            code: StatusCode::OK.as_u16(),
        }),
    ));
}

use std::error::Error;

use crate::util::jwt;
use axum::http::StatusCode;
use bcrypt::{hash, verify, DEFAULT_COST};
use sea_orm::Set;

pub async fn find(
    req: crate::model::user::request::FindRequest,
) -> Result<(Vec<crate::model::user::Model>, u64), ()> {
    let (mut users, total) = crate::model::user::find(
        req.id, req.name, None, req.group, req.email, req.page, req.size,
    )
    .await
    .unwrap();
    for user in users.iter_mut() {
        user.simplify();
    }
    return Ok((users, total));
}

pub async fn create(
    mut req: crate::model::user::request::CreateRequest,
) -> Result<crate::model::user::Model, Box<dyn Error>> {
    let hashed_password = hash(req.password, DEFAULT_COST);
    req.password = hashed_password.unwrap();
    match crate::model::user::create(req.into()).await {
        Ok(mut user) => {
            user.simplify();
            return Ok(user);
        }
        Err(err) => return Err(Box::new(err)),
    }
}

pub async fn update(
    mut req: crate::model::user::request::UpdateRequest,
) -> Result<(), Box<dyn Error>> {
    if let Some(password) = req.password {
        let hashed_password = hash(password, DEFAULT_COST);
        req.password = Some(hashed_password.unwrap());
    }
    match crate::model::user::update(req.into()).await {
        Ok(_) => return Ok(()),
        Err(err) => return Err(Box::new(err)),
    }
}

pub async fn delete(id: i64) -> Result<(), Box<dyn Error>> {
    match crate::model::user::delete(id).await {
        Ok(()) => return Ok(()),
        Err(err) => return Err(Box::new(err)),
    }
}

pub async fn login(
    req: crate::model::user::request::LoginRequest,
) -> Result<(crate::model::user::Model, String), Box<dyn Error>> {
    let (users, total) =
        crate::model::user::find(None, None, Some(req.username), None, None, None, None)
            .await
            .unwrap();

    if total == 0 {
        return Err("user_not_found".into());
    }

    let mut user = users.get(0).unwrap().clone();
    let hashed_password = user.password.clone().unwrap();
    let is_match = verify(&req.password, &hashed_password).unwrap();

    if !is_match {
        return Err("password_incorrect".into());
    }

    let token = jwt::generate_jwt_token(user.id.clone()).await;
    user.simplify();

    return Ok((user, token));
}

pub async fn register(
    req: crate::model::user::request::RegisterRequest,
) -> Result<crate::model::user::Model, StatusCode> {
    match crate::model::user::find(
        None,
        None,
        Some(req.clone().username),
        None,
        None,
        None,
        None,
    )
    .await
    {
        Ok((_, total)) => {
            if total != 0 {
                return Err(StatusCode::CONFLICT);
            }
        }
        Err(_err) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }

    let hashed_password = hash(req.password.clone(), DEFAULT_COST).unwrap();
    let mut user: crate::model::user::ActiveModel = req.into();
    user.password = Set(Some(hashed_password));
    user.group = Set("user".to_string());

    match crate::model::user::create(user).await {
        Ok(user) => return Ok(user),
        Err(_err) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

use crate::{
    data::{
        delete_administrator_data, delete_user_data, get_all_users_data,
        insert_new_administrator_data, update_administrator_data,
    },
    entities::UserEntity,
    models::{CookieSession, UserData},
};
use actix_web::web;

use super::create_hashed_password_core;

pub async fn get_all_users_core() -> Result<Vec<UserEntity>, String> {
    get_all_users_data().await
}

pub async fn insert_new_administrator_core(
    form: web::Json<UserData>,
) -> Result<Result<u64, String>, String> {
    let mut parsed_user = match UserData::validate(form.into_inner()) {
        Err(err) => return Ok(Err(err)),
        Ok(val) => val,
    };

    match UserData::check_none_values(&parsed_user) {
        Err(err) => return Ok(Err(err)),
        Ok(_) => (),
    };

    let current_password = match parsed_user.password {
        None => return Err(format!("User password is None")),
        Some(val) => val,
    };

    let hashed_password = create_hashed_password_core(current_password.as_str())?;

    parsed_user.password = Some(hashed_password);

    insert_new_administrator_data(parsed_user).await
}

pub async fn update_administrator_core(
    cookie_session: &CookieSession,
    form: web::Json<UserData>,
) -> Result<Result<UserData, String>, String> {
    let mut parsed_user = match UserData::validate(form.into_inner()) {
        Err(err) => return Ok(Err(err)),
        Ok(val) => val,
    };

    let current_password = match &parsed_user.password {
        None => None,
        Some(val) => Some(create_hashed_password_core(val.as_str())?),
    };

    parsed_user.password = current_password;

    match update_administrator_data(cookie_session, &parsed_user).await {
        Err(err) => Err(err),
        Ok(_) => Ok(Ok(parsed_user)),
    }
}

pub async fn delete_user_core(form: web::Json<UserData>) -> Result<Result<u64, String>, String> {
    let user_id = match form.id {
        None => return Ok(Err(format!("User ID is None"))),
        Some(val) => UserData::check_id(val)?,
    };

    match delete_user_data(user_id).await {
        Err(err) => Err(err),
        Ok(val) => Ok(Ok(val)),
    }
}

pub async fn delete_administrator_core(
    form: web::Json<UserData>,
) -> Result<Result<u64, String>, String> {
    let admin_id = match form.id {
        None => return Ok(Err(format!("Admin ID is None"))),
        Some(val) => UserData::check_id(val)?,
    };

    match delete_administrator_data(admin_id).await {
        Err(err) => Err(err),
        Ok(val) => Ok(Ok(val)),
    }
}

use actix_web::web;

use crate::{
    data::{insert_new_user_data, update_user_data},
    models::{CookieSession, UserData},
};

use super::create_hashed_password_core;

pub async fn insert_new_user_core(
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

    insert_new_user_data(parsed_user).await
}

pub async fn update_user_core(
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

    match update_user_data(cookie_session, &parsed_user).await {
        Err(err) => Err(err),
        Ok(result) => match result {
            Err(err) => Ok(Err(err)),
            Ok(_) => Ok(Ok(parsed_user)) 
        }
    }
}

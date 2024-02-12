use actix_web::{patch, post, web, HttpRequest, HttpResponse, Responder};

use crate::{
    controllers::validate_user_cookie_session_controller,
    core::{insert_new_user_core, update_user_core},
    models::UserData,
    resources::{COOKIE_USER_EMAIL, COOKIE_USER_ID, COOKIE_USER_NAME},
    utils::cookie_builder,
};

#[post("/insert_new_user")]
pub async fn insert_new_user_controller(user: web::Json<UserData>) -> impl Responder {
    match insert_new_user_core(user).await {
        Err(err) => HttpResponse::InternalServerError().json(err),
        Ok(val) => match val {
            Err(err) => HttpResponse::BadRequest().json(err),
            Ok(rows) => HttpResponse::Ok().json(rows),
        },
    }
}

#[patch("/update_user")]
pub async fn update_user_controller(
    form: web::Json<UserData>,
    request: HttpRequest,
) -> impl Responder {
    let cookie_session = match validate_user_cookie_session_controller(request).await {
        Err(err) => return HttpResponse::InternalServerError().json(err),
        Ok(result) => match result {
            None => return HttpResponse::Unauthorized().finish(),
            Some(val) => val,
        },
    };

    let parsed_new_details = match update_user_core(&cookie_session, form).await {
        Err(err) => return HttpResponse::InternalServerError().json(err),
        Ok(result) => match result {
            Err(err) => return HttpResponse::BadRequest().json(err),
            Ok(val) => val,
        },
    };

    let new_name = match parsed_new_details.name {
        None => cookie_session.name,
        Some(val) => val,
    };

    let new_email = match parsed_new_details.email {
        None => cookie_session.email,
        Some(val) => val,
    };

    let cookie_user_id = cookie_builder(COOKIE_USER_ID, cookie_session.id.to_string());
    let cookie_user_name = cookie_builder(COOKIE_USER_NAME, new_name);
    let cookie_user_email = cookie_builder(COOKIE_USER_EMAIL, new_email);

    HttpResponse::Ok()
        .cookie(cookie_user_id)
        .cookie(cookie_user_name)
        .cookie(cookie_user_email)
        .finish()
}

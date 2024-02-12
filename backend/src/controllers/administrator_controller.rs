use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse, Responder};

use crate::{
    controllers::validate_administrator_cookie_session_controller,
    core::{
        delete_administrator_core, delete_user_core, get_all_users_core,
        insert_new_administrator_core, update_administrator_core,
    },
    models::UserData,
    resources::{COOKIE_USER_EMAIL, COOKIE_USER_ID, COOKIE_USER_IS_ADMIN, COOKIE_USER_NAME},
    utils::{cookie_builder, cookie_destroyer},
};

#[get("/get_all_users")]
pub async fn get_all_users_controller(request: HttpRequest) -> impl Responder {
    match validate_administrator_cookie_session_controller(request).await {
        Err(err) => return HttpResponse::InternalServerError().json(err),
        Ok(result) => {
            if result.is_none() {
                return HttpResponse::Unauthorized().finish();
            }
        }
    };

    match get_all_users_core().await {
        Err(err) => HttpResponse::InternalServerError().json(err),
        Ok(val) => HttpResponse::Ok().json(val),
    }
}

#[post("/insert_new_administrator")]
pub async fn insert_new_administrator_controller(
    body: web::Json<UserData>,
    request: HttpRequest,
) -> impl Responder {
    match validate_administrator_cookie_session_controller(request).await {
        Err(err) => return HttpResponse::InternalServerError().json(err),
        Ok(result) => {
            if result.is_none() {
                return HttpResponse::Unauthorized().finish();
            }
        }
    };

    match insert_new_administrator_core(body).await {
        Err(err) => HttpResponse::InternalServerError().json(err),
        Ok(val) => match val {
            Err(err) => HttpResponse::BadRequest().json(err),
            Ok(val) => HttpResponse::Ok().json(val),
        },
    }
}

#[patch("/update_administrator")]
pub async fn update_administrator_controller(
    form: web::Json<UserData>,
    request: HttpRequest,
) -> impl Responder {
    let cookie_session = match validate_administrator_cookie_session_controller(request).await {
        Err(err) => return HttpResponse::InternalServerError().json(err),
        Ok(result) => match result {
            None => return HttpResponse::Unauthorized().finish(),
            Some(val) => val,
        },
    };

    let parsed_new_details = match update_administrator_core(&cookie_session, form).await {
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
    let cookie_user_is_admin = cookie_builder(COOKIE_USER_IS_ADMIN, true.to_string());

    HttpResponse::Ok()
        .cookie(cookie_user_id)
        .cookie(cookie_user_name)
        .cookie(cookie_user_email)
        .cookie(cookie_user_is_admin)
        .finish()
}

#[delete("/delete_user")]
pub async fn delete_user_controller(
    body: web::Json<UserData>,
    request: HttpRequest,
) -> impl Responder {
    match validate_administrator_cookie_session_controller(request).await {
        Err(err) => return HttpResponse::InternalServerError().json(err),
        Ok(result) => {
            if result.is_none() {
                return HttpResponse::Unauthorized().finish();
            }
        }
    };

    match delete_user_core(body).await {
        Err(err) => return HttpResponse::InternalServerError().json(err),
        Ok(results) => match results {
            Err(err) => HttpResponse::BadRequest().json(err),
            Ok(val) => HttpResponse::Ok().json(val),
        },
    }
}

#[delete("/delete_administrator")]
pub async fn delete_administrator_controller(request: HttpRequest) -> impl Responder {
    let cookie_session = match validate_administrator_cookie_session_controller(request).await {
        Err(err) => return HttpResponse::InternalServerError().json(err),
        Ok(result) => match result {
            None => return HttpResponse::BadRequest().json("Cookie not found"),
            Some(val) => val,
        },
    };

    let mut user_data = UserData::default();
    user_data.id = Some(cookie_session.id);

    let deleted_rows = match delete_administrator_core(actix_web::web::Json(user_data)).await {
        Err(err) => return HttpResponse::InternalServerError().json(err),
        Ok(results) => match results {
            Err(err) => return HttpResponse::BadRequest().json(err),
            Ok(val) => val,
        },
    };

    let cookie_user_id = cookie_destroyer(COOKIE_USER_ID);
    let cookie_user_name = cookie_destroyer(COOKIE_USER_NAME);
    let cookie_user_email = cookie_destroyer(COOKIE_USER_EMAIL);
    let cookie_user_is_admin = cookie_destroyer(COOKIE_USER_IS_ADMIN);

    HttpResponse::Ok()
        .cookie(cookie_user_id)
        .cookie(cookie_user_name)
        .cookie(cookie_user_email)
        .cookie(cookie_user_is_admin)
        .json(deleted_rows)
}

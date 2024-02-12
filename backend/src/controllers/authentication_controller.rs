use crate::{
    core::{
        login_administrator_core, login_user_core, validate_administrator_cookie_session_core,
        validate_administrator_password_core, validate_user_cookie_session_core,
        validate_user_password_core, verify_order_existence_core,
    },
    models::{CookieSession, PaypalCookieDetails, UserData},
    resources::{
        COOKIE_PAYPAL_ORDER_ID, COOKIE_USER_EMAIL, COOKIE_USER_ID, COOKIE_USER_IS_ADMIN,
        COOKIE_USER_NAME,
    },
    utils::{cookie_builder, cookie_destroyer},
};
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};

pub async fn validate_user_cookie_session_controller(
    request: HttpRequest,
) -> Result<Option<CookieSession>, String> {
    let id = match request.cookie(COOKIE_USER_ID) {
        None => return Ok(None),
        Some(val) => match val.value().parse::<i32>() {
            Err(_) => return Ok(None),
            Ok(val) => val,
        },
    };

    let name = match request.cookie(COOKIE_USER_NAME) {
        None => return Ok(None),
        Some(val) => val.value().to_string(),
    };

    let email = match request.cookie(COOKIE_USER_EMAIL) {
        None => return Ok(None),
        Some(val) => val.value().to_string(),
    };

    let cookie_session = CookieSession { id, name, email };

    match validate_user_cookie_session_core(cookie_session).await {
        Err(err) => Err(err.to_string()),
        Ok(val) => match val {
            None => Ok(None),
            Some(val) => Ok(Some(val)),
        },
    }
}

pub async fn validate_administrator_cookie_session_controller(
    request: HttpRequest,
) -> Result<Option<CookieSession>, String> {
    let id = match request.cookie(COOKIE_USER_ID) {
        None => return Ok(None),
        Some(val) => match val.value().parse::<i32>() {
            Err(_) => return Ok(None),
            Ok(val) => val,
        },
    };

    let name = match request.cookie(COOKIE_USER_NAME) {
        None => return Ok(None),
        Some(val) => val.value().to_string(),
    };

    let email = match request.cookie(COOKIE_USER_EMAIL) {
        None => return Ok(None),
        Some(val) => val.value().to_string(),
    };

    match request.cookie(COOKIE_USER_IS_ADMIN) {
        None => return Ok(None),
        Some(val) => match val.value().parse::<bool>() {
            Err(err) => return Err(err.to_string()),
            Ok(res) => if !res {
                return Ok(None)
            }
        },
    };

    let cookie_session = CookieSession { id, name, email };

    match validate_administrator_cookie_session_core(cookie_session).await {
        Err(err) => Err(err.to_string()),
        Ok(val) => match val {
            None => Ok(None),
            Some(val) => Ok(Some(val)),
        },
    }
}

#[post("/user_login")]
pub async fn login_user_controller(form: web::Json<UserData>) -> impl Responder {
    let user_password = match UserData::unwrap_or_error(form.password.as_ref()) {
        Err(_) => return HttpResponse::BadRequest().json("User password is empty"),
        Ok(val) => val,
    };

    let user_email = match UserData::unwrap_or_error(form.email.as_ref()) {
        Err(_) => return HttpResponse::BadRequest().json("User email is empty"),
        Ok(val) => match UserData::check_email(val.to_string()) {
            Err(err) => return HttpResponse::BadRequest().json(err),
            Ok(res) => res,
        },
    };

    match validate_user_password_core(&user_password, &user_email).await {
        Err(err) => return HttpResponse::InternalServerError().json(err),
        Ok(val) => {
            if !val {
                return HttpResponse::Unauthorized().finish();
            }
        }
    };

    let user_details = match login_user_core(&user_email).await {
        Err(err) => return HttpResponse::InternalServerError().json(err),
        Ok(val) => match val {
            None => return HttpResponse::Unauthorized().finish(),
            Some(user_details) => user_details,
        },
    };

    let cookie_user_id = cookie_builder(COOKIE_USER_ID, user_details.0.to_string());
    let cookie_user_name = cookie_builder(COOKIE_USER_NAME, user_details.1);
    let cookie_user_email = cookie_builder(COOKIE_USER_EMAIL, user_email);

    HttpResponse::Ok()
        .cookie(cookie_user_id)
        .cookie(cookie_user_name)
        .cookie(cookie_user_email)
        .finish()
}

#[post("/administrator_login")]
pub async fn login_administrator_controller(form: web::Json<UserData>) -> impl Responder {
    let administrator_password = match UserData::unwrap_or_error(form.password.as_ref()) {
        Err(_) => return HttpResponse::BadRequest().json("User password is empty"),
        Ok(val) => val,
    };

    let administrator_email = match UserData::unwrap_or_error(form.email.as_ref()) {
        Err(_) => return HttpResponse::BadRequest().json("User email is empty"),
        Ok(val) => match UserData::check_email(val.to_string()) {
            Err(err) => return HttpResponse::BadRequest().json(err),
            Ok(res) => res,
        },
    };

    match validate_administrator_password_core(&administrator_password, &administrator_email).await
    {
        Err(err) => return HttpResponse::InternalServerError().json(err),
        Ok(val) => {
            if !val {
                return HttpResponse::Unauthorized().finish();
            }
        }
    };

    let administrator_details = match login_administrator_core(&administrator_email).await {
        Err(err) => return HttpResponse::InternalServerError().json(err),
        Ok(val) => match val {
            None => return HttpResponse::Unauthorized().finish(),
            Some(administrator_details) => administrator_details,
        },
    };

    let cookie_user_id = cookie_builder(COOKIE_USER_ID, administrator_details.0.to_string());
    let cookie_user_name = cookie_builder(COOKIE_USER_NAME, &administrator_details.1);
    let cookie_user_email = cookie_builder(COOKIE_USER_EMAIL, administrator_email);
    let cookie_user_is_admin = cookie_builder(COOKIE_USER_IS_ADMIN, true.to_string());

    HttpResponse::Ok()
        .cookie(cookie_user_id)
        .cookie(cookie_user_name)
        .cookie(cookie_user_email)
        .cookie(cookie_user_is_admin)
        .finish()
}

#[post("/verify_order_existence")]
pub async fn verify_order_existence_controller(request: HttpRequest) -> impl Responder {
    let email = match request.cookie(COOKIE_USER_EMAIL) {
        None => return HttpResponse::Unauthorized().finish(),
        Some(val) => val.value().to_string(),
    };

    let order_id = match request.cookie(COOKIE_PAYPAL_ORDER_ID) {
        None => return HttpResponse::Unauthorized().finish(),
        Some(val) => val.value().to_string(),
    };

    let paypal_cookie_details: PaypalCookieDetails = PaypalCookieDetails { email, order_id };

    match verify_order_existence_core(paypal_cookie_details).await {
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
        Ok(val) => match val {
            None => HttpResponse::Unauthorized().finish(),
            Some(val) => HttpResponse::Ok().json(val),
        },
    }
}

#[post("/user_logout")]
pub async fn logout_user_controller() -> impl Responder {
    let cookie_user_id = cookie_destroyer(COOKIE_USER_ID);
    let cookie_user_name = cookie_destroyer(COOKIE_USER_NAME);
    let cookie_user_email = cookie_destroyer(COOKIE_USER_EMAIL);

    HttpResponse::Ok()
        .cookie(cookie_user_id)
        .cookie(cookie_user_name)
        .cookie(cookie_user_email)
        .finish()
}

#[post("/admin_logout")]
pub async fn logout_admin_controller() -> impl Responder {
    let cookie_user_id = cookie_destroyer(COOKIE_USER_ID);
    let cookie_user_name = cookie_destroyer(COOKIE_USER_NAME);
    let cookie_user_email = cookie_destroyer(COOKIE_USER_EMAIL);
    let cookie_user_is_admin = cookie_destroyer(COOKIE_USER_IS_ADMIN);

    HttpResponse::Ok()
        .cookie(cookie_user_id)
        .cookie(cookie_user_name)
        .cookie(cookie_user_email)
        .cookie(cookie_user_is_admin)
        .finish()
}

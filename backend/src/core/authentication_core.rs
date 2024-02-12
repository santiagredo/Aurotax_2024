use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

use crate::{
    data::{
        login_administrator_data, login_user_data, validate_administrator_cookie_session_data,
        validate_administrator_password_data, validate_user_cookie_session_data,
        validate_user_password_data, verify_order_existence_data,
    },
    models::{CookieSession, PaypalCookieDetails, UserData},
    utils::{has_invalid_chars, is_empty, is_too_short},
};

pub fn create_hashed_password_core(password: &str) -> Result<String, String> {
    is_empty(password.to_string())?;
    has_invalid_chars(password.to_string())?;
    is_too_short(password.to_string())?;

    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => Ok(hash.to_string()),
        Err(err) => Err(err.to_string()),
    }
}

pub async fn validate_user_password_core(
    user_password: &str,
    user_email: &str,
) -> Result<bool, String> {
    let checked_user_email = match UserData::check_email(user_email.to_string()) {
        Err(_) => return Ok(false),
        Ok(val) => val,
    };

    let checked_user_password = match UserData::check_password(user_password.to_string()) {
        Err(_) => return Ok(false),
        Ok(val) => val,
    };

    let stored_hash = match validate_user_password_data(&checked_user_email.as_str()).await? {
        None => return Ok(false),
        Some(val) => val,
    };

    let parsed_hash = match PasswordHash::new(&stored_hash) {
        Err(err) => return Err(err.to_string()),
        Ok(val) => val,
    };

    Ok(Argon2::default()
        .verify_password(checked_user_password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub async fn validate_administrator_password_core(
    administrator_password: &str,
    administrator_email: &str,
) -> Result<bool, String> {
    let checked_administrator_email = match UserData::check_email(administrator_email.to_string()) {
        Err(_) => return Ok(false),
        Ok(val) => val,
    };

    let checked_administrator_password =
        match UserData::check_password(administrator_password.to_string()) {
            Err(_) => return Ok(false),
            Ok(val) => val,
        };

    let stored_hash =
        match validate_administrator_password_data(&checked_administrator_email.as_str()).await? {
            None => return Ok(false),
            Some(val) => val,
        };

    let parsed_hash = match PasswordHash::new(&stored_hash) {
        Err(err) => return Err(err.to_string()),
        Ok(val) => val,
    };

    Ok(Argon2::default()
        .verify_password(checked_administrator_password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub async fn validate_user_cookie_session_core(
    cookie_session: CookieSession,
) -> Result<Option<CookieSession>, String> {
    let id = UserData::check_id(cookie_session.id)?;
    let name = UserData::check_name(cookie_session.name)?;
    let email = UserData::check_email(cookie_session.email)?;

    let checked_cookie_session = CookieSession { id, name, email };

    match validate_user_cookie_session_data(&checked_cookie_session).await {
        Err(err) => return Err(err),
        Ok(val) => match val {
            None => return Ok(None),
            Some(_) => Ok(Some(checked_cookie_session)),
        },
    }
}

pub async fn validate_administrator_cookie_session_core(
    cookie_session: CookieSession,
) -> Result<Option<CookieSession>, String> {
    let id = UserData::check_id(cookie_session.id)?;
    let name = UserData::check_name(cookie_session.name)?;
    let email = UserData::check_email(cookie_session.email)?;

    let checked_cookie_session = CookieSession { id, name, email };

    match validate_administrator_cookie_session_data(&checked_cookie_session).await {
        Err(err) => return Err(err),
        Ok(val) => match val {
            None => return Ok(None),
            Some(_) => Ok(Some(checked_cookie_session)),
        },
    }
}

pub async fn login_user_core(user_email: &str) -> Result<Option<(i32, String)>, String> {
    let checked_email = UserData::check_email(user_email.to_string())?;

    login_user_data(checked_email.as_str()).await
}

pub async fn login_administrator_core(
    administrator_email: &str,
) -> Result<Option<(i32, String)>, String> {
    let checked_email = UserData::check_email(administrator_email.to_string())?;

    login_administrator_data(checked_email.as_str()).await
}

pub async fn verify_order_existence_core(
    paypal_cookie_details: PaypalCookieDetails,
) -> Result<Option<PaypalCookieDetails>, String> {
    verify_order_existence_data(paypal_cookie_details).await
}

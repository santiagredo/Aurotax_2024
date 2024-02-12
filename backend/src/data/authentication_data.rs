use crate::{
    database::get_db_conn,
    models::{CookieSession, PaypalCookieDetails},
};

pub async fn validate_user_password_data(user_email: &str) -> Result<Option<String>, String> {
    let mut conn = get_db_conn().await?;

    match sqlx::query!(
        "SELECT usr_password_hash FROM users WHERE usr_email = $1",
        user_email
    )
    .fetch_optional(&mut *conn)
    .await
    {
        Err(err) => Err(err.to_string()),
        Ok(row) => match row {
            None => Ok(None),
            Some(val) => Ok(Some(val.usr_password_hash)),
        },
    }
}

pub async fn validate_administrator_password_data(
    administrator_email: &str,
) -> Result<Option<String>, String> {
    let mut conn = get_db_conn().await?;

    match sqlx::query!(
        "SELECT adm_password_hash FROM administrators WHERE adm_email = $1",
        administrator_email
    )
    .fetch_optional(&mut *conn)
    .await
    {
        Err(err) => Err(err.to_string()),
        Ok(row) => match row {
            None => Ok(None),
            Some(val) => Ok(Some(val.adm_password_hash)),
        },
    }
}

pub async fn validate_administrator_cookie_session_data(
    cookie_session: &CookieSession,
) -> Result<Option<bool>, String> {
    let mut conn = get_db_conn().await?;

    match sqlx::query!(
        "
        SELECT adm_administrator_id 
        FROM administrators 
        WHERE adm_administrator_id = $1 
            AND adm_name = $2 
            AND adm_email = $3
        ",
        cookie_session.id,
        cookie_session.name,
        cookie_session.email
    )
    .fetch_optional(&mut *conn)
    .await
    {
        Err(err) => Err(err.to_string()),
        Ok(row) => match row {
            None => Ok(None),
            Some(_) => Ok(Some(true)),
        },
    }
}

pub async fn validate_user_cookie_session_data(
    cookie_session: &CookieSession,
) -> Result<Option<bool>, String> {
    let mut conn = get_db_conn().await?;

    match sqlx::query!(
        "
        SELECT usr_user_id 
        FROM users 
        WHERE usr_user_id = $1 
            AND usr_name = $2 
            AND usr_email = $3
        ",
        cookie_session.id,
        cookie_session.name,
        cookie_session.email
    )
    .fetch_optional(&mut *conn)
    .await
    {
        Err(err) => Err(err.to_string()),
        Ok(row) => match row {
            None => Ok(None),
            Some(_) => Ok(Some(true)),
        },
    }
}

pub async fn login_user_data(user_email: &str) -> Result<Option<(i32, String)>, String> {
    let mut conn = get_db_conn().await?;

    match sqlx::query!(
        "SELECT usr_user_id, usr_name FROM users WHERE usr_email = $1",
        user_email
    )
    .fetch_optional(&mut *conn)
    .await
    {
        Err(err) => Err(err.to_string()),
        Ok(val) => match val {
            None => Ok(None),
            Some(val) => Ok(Some((val.usr_user_id, val.usr_name))),
        },
    }
}

pub async fn login_administrator_data(
    administrator_email: &str,
) -> Result<Option<(i32, String)>, String> {
    let mut conn = get_db_conn().await?;

    match sqlx::query!(
        "SELECT adm_administrator_id, adm_name FROM administrators WHERE adm_email = $1",
        administrator_email
    )
    .fetch_optional(&mut *conn)
    .await
    {
        Err(err) => Err(err.to_string()),
        Ok(val) => match val {
            None => Ok(None),
            Some(val) => Ok(Some((val.adm_administrator_id, val.adm_name))),
        },
    }
}

pub async fn verify_order_existence_data(
    paypal_cookie_details: PaypalCookieDetails,
) -> Result<Option<PaypalCookieDetails>, String> {
    let mut conn = get_db_conn().await?;

    match sqlx::query!(
        "
        SELECT ord_paypal_order_id, ord_payer_email 
        FROM paypal_orders 
        WHERE ord_paypal_order_id = $1 
            AND ord_payer_email = $2
        ",
        paypal_cookie_details.order_id,
        paypal_cookie_details.email
    )
    .fetch_optional(&mut *conn)
    .await
    {
        Err(err) => Err(err.to_string()),
        Ok(val) => match val {
            None => Ok(None),
            Some(val) => {
                let stored_order_details = PaypalCookieDetails {
                    order_id: val.ord_paypal_order_id,
                    email: val.ord_payer_email,
                };

                return Ok(Some(stored_order_details));
            }
        },
    }
}

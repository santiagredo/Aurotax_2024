use crate::{
    database::get_db_conn,
    models::{CookieSession, UserData},
};

pub async fn insert_new_user_data(user: UserData) -> Result<Result<u64, String>, String> {
    let mut conn = get_db_conn().await?;

    let query = match sqlx::query(
        "INSERT INTO users (usr_name, usr_email, usr_password_hash) VALUES ($1, $2, $3)",
    )
    .bind(user.name)
    .bind(user.email)
    .bind(user.password)
    .execute(&mut *conn)
    .await
    {
        Err(err) => err,
        Ok(rows) => return Ok(Ok(rows.rows_affected())),
    };

    match query.as_database_error() {
        Some(err) => match err {
            err if err.is_unique_violation() => Ok(Err(err.to_string())),
            _ => Err(err.to_string()),
        },
        None => Err(query.to_string()),
    }
}

pub async fn update_user_data(
    cookie_session: &CookieSession,
    user_data: &UserData,
) -> Result<Result<u64, String>, String> {
    let mut conn = get_db_conn().await?;

    let query = match sqlx::query!(
        "
        UPDATE users
        SET usr_email = COALESCE($1, usr_email), 
            usr_name = COALESCE($2, usr_name),
            usr_password_hash = COALESCE($3, usr_password_hash)
        WHERE usr_user_id = $4 
            and usr_email = $5
        ",
        user_data.email,
        user_data.name,
        user_data.password,
        cookie_session.id,
        cookie_session.email
    )
    .execute(&mut *conn)
    .await
    {
        Err(err) => err,
        Ok(val) => return Ok(Ok(val.rows_affected())),
    };

    match query.as_database_error() {
        Some(err) => match err {
            _ => Err(err.to_string()),
        },
        None => Err(query.to_string()),
    }
}

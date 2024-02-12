use crate::{
    database::get_db_conn,
    entities::UserEntity,
    models::{CookieSession, UserData},
};
use sqlx::Row;

pub async fn get_all_users_data() -> Result<Vec<UserEntity>, String> {
    let mut conn = get_db_conn().await?;

    match sqlx::query("SELECT * FROM users")
        .fetch_all(&mut *conn)
        .await
    {
        Err(err) => Err(format!("{err}")),
        Ok(rows) => {
            let all_users: Vec<UserEntity> = rows
                .into_iter()
                .map(|x| UserEntity {
                    usr_user_id: x.get("usr_user_id"),
                    usr_email: x.get("usr_email"),
                    usr_name: x.get("usr_name"),
                    usr_password_hash: String::from("Secret password hash"),
                })
                .collect();

            Ok(all_users)
        }
    }
}

pub async fn insert_new_administrator_data(user: UserData) -> Result<Result<u64, String>, String> {
    let mut conn = get_db_conn().await?;

    let query = match sqlx::query(
        "INSERT INTO administrators (adm_name, adm_email, adm_password_hash) VALUES ($1, $2, $3)",
    )
    .bind(user.name.as_ref())
    .bind(user.email.as_ref())
    .bind(user.password.as_ref())
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

pub async fn update_administrator_data(
    cookie_session: &CookieSession,
    user_data: &UserData,
) -> Result<u64, String> {
    let mut conn = get_db_conn().await?;

    let query = match sqlx::query!(
        "
        UPDATE administrators
        SET adm_email = COALESCE($1, adm_email), 
            adm_name = COALESCE($2, adm_name),
            adm_password_hash = COALESCE($3, adm_password_hash)
        WHERE adm_administrator_id = $4 
            and adm_email = $5
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
        Ok(val) => return Ok(val.rows_affected()),
    };

    match query.as_database_error() {
        Some(err) => match err {
            _ => Err(err.to_string()),
        },
        None => Err(query.to_string()),
    }
}

pub async fn delete_user_data(user_id: i32) -> Result<u64, String> {
    let mut conn = get_db_conn().await?;

    match sqlx::query!("DELETE FROM users WHERE usr_user_id = $1", user_id).execute(&mut *conn).await {
        Err(err) => Err(format!("{err}")),
        Ok(val) => Ok(val.rows_affected()),
    }
}

pub async fn delete_administrator_data(admin_id: i32) -> Result<u64, String> {
    let mut conn = get_db_conn().await?;

    match sqlx::query!("DELETE FROM administrators WHERE adm_administrator_id = $1", admin_id).execute(&mut *conn).await {
        Err(err) => Err(format!("{err}")),
        Ok(val) => Ok(val.rows_affected()),
    }
}

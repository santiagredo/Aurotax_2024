use sqlx::Row;

use crate::database::get_db_conn;

pub async fn health_check_data() -> Result<String, String> {
    let mut conn = match get_db_conn().await {
        Ok(conn) => conn,
        Err(_) => return Err("Unable to connect to database".to_string())
    };

    let rows = match sqlx::query("SELECT * FROM health_check")
        .fetch_one(&mut *conn)
        .await {
            Ok(rows) => rows,
            Err(_) => return Err("Database table not found".to_string())
        };

    match rows.try_get("message") {
        Ok(val) => return Ok(val),
        Err(_) => return Err("Row not found".to_string())
    }
}
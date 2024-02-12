use sqlx::{PgPool, Pool, Postgres};
use once_cell::sync::OnceCell;

use crate::configuration::get_configuration;

static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

async fn get_db_pool() {
    let configuration = get_configuration().db_connection_string();

    let pg_pool = PgPool::connect(configuration.as_str())
        .await
        .unwrap();

    DB_POOL.get_or_init(|| pg_pool);
}

pub async fn get_db_conn() -> Result<sqlx::pool::PoolConnection<Postgres>, String> {
    if DB_POOL.get().is_none() {
        get_db_pool().await
    }

    match DB_POOL.get().unwrap().acquire().await {
        Ok(val) => Ok(val),
        Err(err) => Err(format!("Error connecting to DB: {err}"))
    }
}
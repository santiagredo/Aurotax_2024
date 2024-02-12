use paypal_rs::data::orders::{Order, Payer, OrderStatus};
use sqlx::types::chrono::DateTime;

use crate::database::get_db_conn;

pub async fn capture_order_data(order: &Order) -> Result<Result<u64, String>, String> {
    let mut conn = get_db_conn().await?;

    let payer = match &order.payer {
        None => Payer::default(),
        Some(val) => val.clone(),
    };

    let order_status = match order.status {
        OrderStatus::Completed => "COMPLETED",
        OrderStatus::Created => "CREATED",
        OrderStatus::Saved => "SAVED",
        _ => "OTHER",
    };

    let payer_name = match payer.name {
        None => String::default(),
        Some(val) => format!("{} {}", val.given_name, val.surname)
    };

    let payer_email = match payer.email_address {
        None => String::default(),
        Some(val) => val,
    };

    let create_time = match order.create_time {
        None => DateTime::default(),
        Some(val) => val,
    };

    let update_time = match order.create_time {
        None => DateTime::default(),
        Some(val) => val,
    };

    let query = match sqlx::query!(
        "INSERT INTO paypal_orders (
            ord_paypal_order_id, 
            ord_order_status, 
            ord_payer_name, 
            ord_payer_email, 
            ord_payer_paypal_id, 
            ord_order_create_time, 
            ord_order_update_time
        ) VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7
        )",
        order.id,
        order_status,
        payer_name.to_lowercase(),
        payer_email,
        payer.payer_id,
        create_time,
        update_time
    ).execute(&mut *conn).await {
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

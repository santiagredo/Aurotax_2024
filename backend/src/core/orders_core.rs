use paypal_rs::data::orders::Order;

use crate::data::capture_order_data;

pub async fn capture_order_core(order: &Order) -> Result<Result<u64, String>, String> {
    match capture_order_data(order).await {
        Err(err) => Err(err.to_string()),
        Ok(val) => match val {
            Err(err) => Ok(Err(err.to_string())),
            Ok(val) => Ok(Ok(val)),
        },
    }
}

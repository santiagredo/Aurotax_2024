use crate::data::health_check_data;

pub async fn health_check_core() -> Result<String, String> {
    health_check_data().await
}
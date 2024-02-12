use actix_web::{Responder, HttpResponse, get};
use crate::core::health_check_core;

#[get("/health_check")]
pub async fn health_check_controller() -> impl Responder {
    match health_check_core().await {
        Ok(val) => {
            HttpResponse::Ok().body(val)
        },
        Err(err) => {
            HttpResponse::InternalServerError().body(err)
        }
    }
}
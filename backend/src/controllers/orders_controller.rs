use actix_web::{post, web, HttpResponse, Responder};
use paypal_rs::{
    api::orders::{CaptureOrder, CreateOrder},
    data::{
        common::Currency,
        orders::{Amount, Intent, OrderPayloadBuilder, Payer, PurchaseUnit},
    },
    Client, PaypalEnv,
};

use crate::{
    configuration::get_configuration,
    core::capture_order_core,
    resources::{COOKIE_PAYPAL_ORDER_ID, COOKIE_USER_EMAIL},
    utils::cookie_builder,
};

#[post("/create_order")]
pub async fn create_order_controller() -> impl Responder {
    let (paypal_client_id, paypal_api_secret, _) = get_configuration().get_paypal_credentials();

    let paypal_environment = match get_configuration().is_prod_environment() {
        true => PaypalEnv::Live,
        false => PaypalEnv::Sandbox,
    };

    let mut client = Client::new(paypal_client_id, paypal_api_secret, paypal_environment);

    match client.get_access_token().await {
        Err(err) => return HttpResponse::InternalServerError().json(err.to_string()),
        Ok(_) => (),
    };

    let order = match OrderPayloadBuilder::default()
        .intent(Intent::Capture)
        .purchase_units(vec![PurchaseUnit::new(Amount::new(Currency::USD, "1.0"))])
        .build()
    {
        Err(err) => return HttpResponse::InternalServerError().json(err.to_string()),
        Ok(val) => val,
    };

    let create_order = CreateOrder::new(order);

    let order_created = match client.execute(&create_order).await {
        Err(err) => return HttpResponse::InternalServerError().json(err.to_string()),
        Ok(val) => val,
    };

    HttpResponse::Ok().json(order_created)
}

#[post("/capture_order/{order_id}")]
pub async fn capture_order_controller(path: web::Path<String>) -> impl Responder {
    let order_id = path.into_inner();

    let (paypal_client_id, paypal_api_secret, _) = get_configuration().get_paypal_credentials();

    let paypal_environment = match get_configuration().is_prod_environment() {
        true => PaypalEnv::Live,
        false => PaypalEnv::Sandbox,
    };

    let mut client = Client::new(paypal_client_id, paypal_api_secret, paypal_environment);

    match client.get_access_token().await {
        Err(err) => return HttpResponse::InternalServerError().json(err.to_string()),
        Ok(_) => (),
    };

    let capture_order = CaptureOrder::new(&order_id);

    let order_created = match client.execute(&capture_order).await {
        Err(err) => return HttpResponse::InternalServerError().json(err.to_string()),
        Ok(val) => val,
    };

    match capture_order_core(&order_created).await {
        Err(err) => {
            return HttpResponse::InternalServerError()
                .json(format!("{err} - Please contact administrator"))
        }
        Ok(_) => (),
    };

    let payer = match &order_created.payer {
        None => Payer::default(),
        Some(val) => val.clone(),
    };

    let payer_email = match payer.email_address {
        None => String::default(),
        Some(val) => val,
    };

    let mut paypal_order_id_cookie = cookie_builder(COOKIE_PAYPAL_ORDER_ID, &order_id);
    paypal_order_id_cookie.set_http_only(false);
    let mut user_email_cookie = cookie_builder(COOKIE_USER_EMAIL, &payer_email);
    user_email_cookie.set_http_only(false);

    HttpResponse::Ok()
        .cookie(paypal_order_id_cookie)
        .cookie(user_email_cookie)
        .json(order_created)
}

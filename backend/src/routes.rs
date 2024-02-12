use std::path::PathBuf;

use crate::controllers::{
    capture_order_controller, create_order_controller, delete_administrator_controller, delete_user_controller, get_all_users_controller, health_check_controller::health_check_controller, insert_new_administrator_controller, insert_new_user_controller, login_administrator_controller, login_user_controller, logout_admin_controller, logout_user_controller, update_administrator_controller, update_user_controller, verify_order_existence_controller
};
use actix_files::NamedFile;
use actix_web::{
    get,
    web::{self},
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check_controller)
        .service(
            web::scope("/users")
                .service(insert_new_user_controller)
                .service(update_user_controller),
        )
        .service(
            web::scope("/administrator")
                .service(get_all_users_controller)
                .service(insert_new_administrator_controller)
                .service(update_administrator_controller)
                .service(delete_user_controller)
                .service(delete_administrator_controller),
        )
        .service(
            web::scope("/authentication")
                .service(login_user_controller)
                .service(login_administrator_controller)
                .service(verify_order_existence_controller)
                .service(logout_admin_controller)
                .service(logout_user_controller),
        )
        .service(
            web::scope("/orders")
                .service(create_order_controller)
                .service(capture_order_controller),
        )
        .service(serve_html)
        .service(serve_files);
}

#[get("/")]
async fn serve_html() -> NamedFile {
    let path: PathBuf = format!("../frontend/dist/index.html").parse().unwrap();

    NamedFile::open(path).unwrap()
}

#[get("/{file}")]
async fn serve_files(path: web::Path<String>) -> NamedFile {
    let file_name = path.into_inner();

    let path: PathBuf = format!("../frontend/dist/{file_name}").parse().unwrap();

    NamedFile::open(path).unwrap()
}

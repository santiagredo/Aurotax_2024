use actix_cors::Cors;
use actix_web::{dev::Server, App, HttpServer};

use crate::{configuration::get_configuration, routes::config};

pub fn run() -> Result<Server, std::io::Error> {
    let address = get_configuration().app_local_address();

    println!("Application started on {address}");
    get_configuration().print_environtment();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin()
                    .supports_credentials()
                    .max_age(3600),
            )
            .configure(config)
    })
    .bind(&address)?
    .run();

    Ok(server)
}

use actix_cors::Cors;
use actix_web::{App, HttpServer};

mod controllers;
mod services;

use crate::controllers::get_volume::get_volume;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting...");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive().allow_any_origin())
            .service(get_volume)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

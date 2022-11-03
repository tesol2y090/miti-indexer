use actix_cors::Cors;
use actix_web::{App, HttpServer};

mod controllers;
mod services;
mod utils;

use crate::controllers::get_market_data::get_market_data;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting...");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive().allow_any_origin())
            .service(get_market_data)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

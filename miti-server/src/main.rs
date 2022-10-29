use actix_cors::Cors;
use actix_web::{
    get,
    web::{self},
    App, HttpServer, Responder, Result,
};

mod services;

use crate::services::gql::mintbase_gql_client::get_sale_volume;

#[get("/get_volume")]
async fn greet() -> Result<impl Responder> {
    let date = "2022-10-20".to_string();
    let kind = "sale".to_string();

    let volume = get_sale_volume(date, kind).await.unwrap();
    Ok(web::Json(volume))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting...");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive().allow_any_origin())
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

use actix_cors::Cors;
use actix_web::{
    get,
    web::{self},
    App, HttpServer, Responder,
};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
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

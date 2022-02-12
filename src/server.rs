// Crates
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use std::string::String;

#[actix_web::main]
pub async fn main_server(ipaddr: String, ipport: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind((ipaddr, ipport))?
    .run()
    .await
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}
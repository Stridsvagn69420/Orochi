// Crates
extern crate env_logger;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, middleware::Logger};
use std::string::String;

#[actix_web::main]
pub async fn main_server(ipaddr: String, ipport: u16) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("[%{r}a]:%s %r (%{User-Agent}i)"))
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
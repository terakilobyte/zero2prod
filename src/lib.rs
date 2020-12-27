use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::env;

extern crate log;

#[get("/")]
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[get("/health_check")]
async fn health_check(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn run() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| App::new().service(health_check).service(greet))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

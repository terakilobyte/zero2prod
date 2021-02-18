use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::env;

extern crate log;

async fn health_check(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

async fn hello_name(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    format!("Hello, {}", name)
}

fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .route("/health_check", web::get().to(health_check))
            .route("/hello/{name}", web::get().to(hello_name)),
    );
}

pub fn run() -> Result<Server, std::io::Error> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let server = HttpServer::new(|| App::new().configure(app_config))
        .bind("127.0.0.1:8080")?
        .run();

    Ok(server)
}

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
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

pub async fn run() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| App::new().configure(app_config))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use crate::*;

    use actix_web::dev::{HttpResponseBuilder, Service, ServiceResponse};
    use actix_web::http::StatusCode;
    use actix_web::test::{self};
    use actix_web::{App, Responder};

    #[actix_rt::test]
    async fn health_check_unit_test() {
        let req = test::TestRequest::default().to_http_request();
        let result = health_check(req.clone()).await;
        let resp = match result.respond_to(&req).await {
            Ok(t) => t,
            Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
        };
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn health_check_integration_test() {
        let app = test::init_service(App::new().configure(app_config)).await;
        let req = test::TestRequest::get().uri("/health_check").to_request();
        let resp: ServiceResponse = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }
}

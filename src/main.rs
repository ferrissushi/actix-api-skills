use actix::main;
use actix_web::{
    web::{self, get, head, ServiceConfig},
    App, HttpResponse, HttpServer, Responder,
};

async fn default_service() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

async fn not_allowed() -> impl Responder {
    HttpResponse::MethodNotAllowed()
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/path")
            .route(get().to(default_service))
            .route(head().to(not_allowed)),
    );
}

fn scoped_config(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/hello").route(get().to(hello)));
}

#[main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

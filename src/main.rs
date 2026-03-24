use actix_web::{
    get, main,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};

async fn health() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").route("/health", web::get().to(health)))
            .service(hello)
    })
    .workers(4)
    .shutdown_timeout(100)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

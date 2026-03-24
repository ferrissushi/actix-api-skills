use actix_web::{
    main,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};

async fn health() -> impl Responder {
    HttpResponse::Ok()
}

#[main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::scope("/api").route("/health", web::get().to(health)))
    })
    .workers(4)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

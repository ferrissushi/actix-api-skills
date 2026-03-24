use actix_web::{main, web, App, HttpResponse, HttpServer, Responder};

async fn health() -> impl Responder {
    HttpResponse::Ok()
}

#[main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(health)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

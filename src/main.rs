use actix_web::{main, web::get, App, HttpResponse, HttpServer};

#[main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", get().to(HttpResponse::Ok())))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

use actix_web::{guard, main, web, App, HttpResponse, HttpServer};

#[main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/")
                    .guard(guard::Host("www.rust-lang.org"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("www") })),
            )
            .route("/", web::to(HttpResponse::Ok))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

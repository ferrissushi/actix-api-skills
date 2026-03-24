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

#[get("/hello/{name}")]
async fn greeting(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    let msg = format!("Hello {}", name);
    HttpResponse::Ok().body(msg)
}

#[main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").route("/health", web::get().to(health)))
            .service(hello)
            .service(greeting)
    })
    .workers(4)
    .shutdown_timeout(100)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

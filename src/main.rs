use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct QueryParam {
    page: i32,
    limit: i32,
}

#[get("/hello")]
async fn hello(pagination: web::Query<QueryParam>) -> impl Responder {
    let page = pagination.page;
    let limit = pagination.limit;
    let msg = format!("Page is {} and limit is {}", page, limit);
    HttpResponse::Ok().body(msg)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

use std::net::Ipv4Addr;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_info = ServerInfo {
        adress: Ipv4Addr::new(127, 0, 0, 1),
        port: 8080,
    };
    let server_adress = server_info.adress;
    let server_port = server_info.port;
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((server_adress, server_port))?
    .run()
    .await
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ServerInfo {
    adress: Ipv4Addr,
    port: u16,
}


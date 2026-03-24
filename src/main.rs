use actix_web::{main, post, web::Json, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    username: String,
    password: String,
}

#[post("/users")]
async fn create_user(user: Json<User>) -> impl Responder {
    let msg = format!(
        "User credentials: {} with password {}",
        user.username, user.password
    );
    HttpResponse::Created().body(msg)
}

#[main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(create_user))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

use actix_web::{main, post, web::Json, App, HttpResponse, HttpServer, Responder, get};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
struct User {
    username: String,
    password: String,
}

impl Default for User {
    fn default() -> Self {
        Self { username: "John".to_string(), password: "        ".to_string()}
    }
}

#[post("/users")]
async fn create_user(user: Json<User>) -> impl Responder {
    let msg = format!(
        "User credentials: {} with password {}",
        user.username, user.password
    );
    HttpResponse::Created().body(msg)
}

#[get("/users")]
async fn get_user() -> impl Responder {
    let user: User = Default::default();
    HttpResponse::Ok().json(user)
}

#[main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(create_user).service(get_user))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

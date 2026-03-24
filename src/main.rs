use std::sync::Mutex;

use actix_web::{
    get, post,
    web::{Data, Json},
    App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
struct User {
    username: Mutex<String>,
    password: Mutex<String>,
}

#[get("/users")]
async fn get_users(users: Data<Mutex<Vec<User>>>) -> impl Responder {
    HttpResponse::Ok().json(users)
}

#[post("/users")]
async fn create_user(users: Data<Mutex<Vec<User>>>, user: Json<User>) -> impl Responder {
    users.lock().unwrap().push(user.into_inner());
    HttpResponse::Created().body("user created")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_users).service(create_user))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

use std::sync::Mutex;

use actix::main;
use actix_web::{
    get, post,
    web::{scope, Data, Json},
    App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};

struct AppState {
    users: Mutex<Vec<User>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct User {
    username: String,
    password: Option<String>,
    age: u8,
    sexe: Sexe,
}

impl Default for User {
    fn default() -> Self {
        Self {
            username: "undefined".to_string(),
            password: Option::None,
            age: 1,
            sexe: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum Sexe {
    Male,
    Female,
    Other,
}

impl Default for Sexe {
    fn default() -> Self {
        Self::Male
    }
}

#[get("/users")]
async fn get_users(data: Data<AppState>) -> impl Responder {
    let users = data.users.lock().unwrap();
    HttpResponse::Ok().json(&*users)
}

#[post("/users")]
async fn post_users(user: Json<User>, data: Data<AppState>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    users.push(user.into_inner());
    HttpResponse::Created().json(&*users)
}

#[main]
async fn main() -> std::io::Result<()> {
    let app_state = Data::new(AppState {
        users: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(scope("/api").service(get_users).service(post_users)) // ← inside closure
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

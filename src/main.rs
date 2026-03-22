use std::sync::Mutex;

use actix_web::{
    web::{get, Data},
    App, HttpServer, Responder,
};

struct AppState {
    counter: Mutex<i32>,
}

async fn index(data: Data<AppState>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("{counter}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = Data::new(AppState {
        counter: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .route("/", get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

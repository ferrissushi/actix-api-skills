use actix_web::{get, web::{self, Data}, App, HttpServer};

struct AppState {
    app_name: String
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().app_data(Data::new(AppState {
            app_name: "state_app".to_string()
        })).service(index)
    })
        .bind(("127.0.0.1", 8080))?
        .run().await
}

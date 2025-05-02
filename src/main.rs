use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};
mod model;
mod handlers;
use handlers::health_check_handler;
mod response;
pub use crate::model::AppState;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let todo_db = AppState::new();
    let app_data = web::Data::new(todo_db);

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:3000/")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(app_data.clone())
            .configure(handlers::config)
            .wrap(cors)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}



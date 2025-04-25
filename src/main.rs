use actix_web::HttpServer;
use serde::Serialize;
mod model;
mod handlers;
use handlers::health_check_handler;
mod response;
pub use crate::model::AppState;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting on port 8080");

    HttpServer::new(|| {
        println!("Server is running");
        actix_web::App::new()
            .service(health_check_handler)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}



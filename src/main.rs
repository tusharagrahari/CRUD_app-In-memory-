use actix_web::{get, Responder, HttpServer};
use serde::Serialize;
mod model;
pub use crate::model::AppState;


#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[get("/health_check")]
pub async fn health_check_handler() -> impl Responder {
    let response = GenericResponse {
        status: "success".to_string(),
        message: "Server is running".to_string(),
    };
    actix_web::HttpResponse::Ok().json(response)
}

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



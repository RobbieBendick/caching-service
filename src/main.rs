use actix_web::{post, web::Json, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(api::set_key_value))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
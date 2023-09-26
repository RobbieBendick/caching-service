use actix_web::{Responder, web::Json, HttpResponse, post};
use serde::{Deserialize, Serialize};
use serde_json::json;



#[derive(Serialize, Deserialize, Debug)]
struct KeyVal {
    key: String,
    value: serde_json::Value,
}

enum ErrorTypes {
    InternalServerError,
    BadRequest,
    Unauthorized,
}

trait ErrorResponse {
    fn to_response(&self) -> HttpResponse;
}

impl ErrorResponse for ErrorTypes {
    fn to_response(&self) -> HttpResponse {
        match self {
            ErrorTypes::BadRequest => HttpResponse::BadRequest()
                .json(json!({ "error": "bad_request", "error_message": "Bad request." })),
            ErrorTypes::InternalServerError => HttpResponse::InternalServerError()
                .json(json!({ "error": "internal_server_error", "error_message": "Internal Server Error." })),
            ErrorTypes::Unauthorized => HttpResponse::Unauthorized()
                .json(json!({ "error": "unauthorized", "error_message": "Unauthorized." })),
        }
    }
}

#[post("/set")]
pub async fn set_key_value(data: Json<KeyVal>) -> impl Responder {

    // let client = redis::Client::open("redis://127.0.0.1").expect("Failed to create Redis client");

    // let mut con = client
    //     .get_connection()
    //     .expect("Failed to get Redis connection");

    // set key value pair on connection
    // redis::cmd("SET").arg(key).arg(value).execute(&mut con);

    let result = if !data.value.is_null()  {
        Ok("Value is okay")
    } else {
        Err(ErrorTypes::BadRequest)
    };

    match result {
        Ok(res) => HttpResponse::Ok().json(json!({ "data": res, "message": "Success" })),
        Err(err) => err.to_response(),
    }
}

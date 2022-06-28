use actix_web::{Responder, web, get, post, delete, HttpResponse, http::{StatusCode, header::ContentType}};

use crate::{model::User, http_utils};

#[get("/users")]
pub async fn list() -> impl Responder {
    "GET /users called.\n"
}

#[get("users/{id}")]
pub async fn get_user_by_id(uid: web::Path<u64>) -> impl Responder {
    let user_id: u64 = uid.into_inner();
    if user_id < 100 {
        let user = User{id: user_id, name: String::from("user")};
        http_utils::user_as_json(&user)
    } else {
        HttpResponse::build(StatusCode::NOT_FOUND)
            .content_type(ContentType::json())
            .finish()
    }
}

#[post("users")]
pub async fn create_user(body_bytes: web::Bytes) -> impl Responder {
    let body = String::from_utf8(body_bytes.to_vec()).unwrap();
    format!("POST /users called.\nHttp body: {body}\n")
}

#[post("users/{id}")]
pub async fn update_user(uid: web::Path<u64>, body_bytes: web::Bytes) -> impl Responder {
    let body = String::from_utf8(body_bytes.to_vec()).unwrap();
    format!("POST /users/{uid} called.\nHttp body: {body}\n")
}

#[delete("/users/{id}")]
pub async fn delete_user(uid: web::Path<u64>) -> impl Responder {
    format!("Delete user {}", uid)
}


use actix_web::{Responder, web, get, post, delete};

#[get("/users")]
pub async fn list() -> impl Responder {
    "GET /users called.\n"
}

#[get("users/{id}")]
pub async fn get_user_by_id(uid: web::Path<u64>) -> impl Responder {
    format!("GET /users/{uid} called.\n")
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


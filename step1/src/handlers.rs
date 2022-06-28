use actix_web::{Responder, web, get, post, delete, HttpResponse, http::header::ContentType};

#[get("/users")]
pub async fn list() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Get users list handler called.\n")
}

#[get("users/{id}")]
pub async fn get_user_by_id(uid: web::Path<u64>) -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(format!("Find user handler called for {uid}.\n"))
}

#[post("users")]
pub async fn create_user(body_bytes: web::Bytes) -> impl Responder {
    let body = String::from_utf8(body_bytes.to_vec()).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(format!("Create user handler called.\nHttp body: {body}\n"))
}

#[post("users/{id}")]
pub async fn update_user(uid: web::Path<u64>, body_bytes: web::Bytes) -> impl Responder {
    let body = String::from_utf8(body_bytes.to_vec()).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(format!("Update user handler with {uid} called.\nHttp body: {body}\n"))
}

#[delete("/users/{id}")]
pub async fn delete_user(uid: web::Path<u64>) -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(format!("Delete user handler called with {uid}"))
}


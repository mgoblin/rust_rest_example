use actix_web::{Responder, web::{self, Data}, get, post, delete, HttpResponse};

use crate::{http_utils, services::{UserInMemoryDAO, UserDAO}, model::User};

#[get("/users")]
pub async fn list(dao: Data<UserInMemoryDAO>) -> impl Responder {
    web::Json(dao.list())
}

#[get("users/{id}")]
pub async fn get_user_by_id(uid: web::Path<u64>, dao: Data<UserInMemoryDAO>) -> impl Responder {
    match dao.find_by_id(uid.into_inner()) {
        Some(u) => http_utils::user_as_json(&u),
        None => http_utils::user_not_found()
    }
}

#[post("users")]
pub async fn create_user(body_bytes: web::Bytes, dao: Data<UserInMemoryDAO>) -> impl Responder {
    let try_body = String::from_utf8(body_bytes.to_vec());
    match try_body {
        Ok(body) => {
            match dao.create(&body) {
                Ok(u) => http_utils::user_as_json(&u),
                Err(e) => http_utils::user_not_modified(&e)
            }
        }, 
        Err(err) => HttpResponse::BadRequest().body(err.to_string())
    }
}

#[post("users/{id}")]
pub async fn update_user(uid: web::Path<u64>, body_bytes: web::Bytes, dao: Data<UserInMemoryDAO>) -> impl Responder {
    let try_body = String::from_utf8(body_bytes.to_vec());
    match try_body {
        Ok(body) => {
            let user = User {id: uid.into_inner(), name: body};
            match dao.update(&user) {
                Ok(u) => http_utils::user_as_json(&u),
                Err(e) => http_utils::user_not_modified(&e)
            }
        },
        Err(err) => HttpResponse::BadRequest().body(err.to_string())
    }
}

#[delete("/users/{id}")]
pub async fn delete_user(uid: web::Path<u64>, dao: Data<UserInMemoryDAO>) -> impl Responder {
    match dao.delete_by_id(uid.into_inner()) {
        Ok(u) => http_utils::user_as_json(&u),
        Err(_) => http_utils::user_not_found()    
    }
}


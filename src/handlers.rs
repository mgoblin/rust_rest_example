use actix_web::{Responder, web, get, post, HttpResponse};
use actix_web::web::{Data, Query};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use rbatis::{PageRequest};
use crate::{UsersService};
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct Pagination {
    page_no: Option<u64>,
    page_size: Option<u64>,
}

#[derive(Debug, Serialize)]
struct UserNotFoundError<'a> {
    id: u64,
    message: &'a str,
}

#[derive(Deserialize)]
pub struct NewUser {
    pub name: String,
}


impl UserNotFoundError<'static> {
    fn new<'a>(user_id: u64) -> UserNotFoundError<'a> {
        UserNotFoundError {
            id: user_id,
            message: "User not found",
        }
    }
}

fn json_response<T: Serialize>(obj: T, http_status: StatusCode) -> HttpResponse {
    HttpResponse::build(http_status)
        .insert_header(ContentType::json())
        .body(serde_json::to_string(&obj).unwrap())
}

#[get("/users")]
pub async fn list(us: Data<UsersService>, page: Query<Pagination>) -> impl Responder {
    let users_service = us.get_ref();
    let p = page.into_inner();
    let users = users_service.list(
        &PageRequest::new(
            p.page_no.unwrap_or(0),
            p.page_size.unwrap_or(10),
        )).await;
    web::Json(users)
}

#[get("users/{id}")]
pub async fn get_user_by_id(us: Data<UsersService>, uid: web::Path<u64>) -> impl Responder {
    let users_service = us.get_ref();
    let user_id = uid.clone();
    let maybe_user = users_service.find_by_id(user_id).await;
    match maybe_user {
        Some(user) => json_response(&user, StatusCode::OK),
        None => json_response(&UserNotFoundError::new(user_id), StatusCode::NOT_FOUND),
    }
}

#[post("users")]
pub async fn create_user(us: Data<UsersService>, nuser: web::Json<NewUser>) -> impl Responder {
    let uname = nuser.into_inner().name;
    let user = us.get_ref().create_user(&uname).await;
    web::Json(user)
}

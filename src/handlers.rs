use actix_web::{Responder, web, get};
use actix_web::web::{Data, Query};
use rbatis::PageRequest;
use crate::UsersService;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pagination {
    page_no: Option<u64>,
    page_size: Option<u64>,
}

#[get("/users")]
pub async fn list(us: Data<UsersService>, page: Query<Pagination>) -> impl Responder {
    let users_service = us.get_ref();
    let p = page.into_inner();
    let users = users_service.list(
        &PageRequest::new(
            p.page_no.unwrap_or(0),
            p.page_size.unwrap_or(10)
        )).await;
    web::Json(users)
}

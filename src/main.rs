#[macro_use]
extern crate rbatis;

use serde::Deserialize;

use fast_log::config::Config;
use rbatis::PageRequest;

use actix_web::{get, web, middleware, App, HttpServer, Responder};
use actix_web::web::{Data, Query};

use crate::model::{Users};
use crate::service::UsersService;

mod migrations;
mod model;
mod service;

#[derive(Debug, Deserialize)]
pub struct Pagination {
    page_no: Option<u64>,
    page_size: Option<u64>,
}

#[get("/users")]
async fn list(us: Data<UsersService>, page: Query<Pagination>) -> impl Responder {
    let users_service = us.get_ref();
    let p = page.into_inner();
    let users = users_service.list(
        &PageRequest::new(
            p.page_no.unwrap_or(0),
            p.page_size.unwrap_or(10)
        )).await;
    web::Json(users)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let host = "localhost";
    let user = "postgres";
    let dbname = "RBatisExample";

    let mconn_string = std::format!("host={} user={} dbname={}", host, user, dbname);
    let rconn_string = std::format!("postgres://{1}@{0}:5432/RBatisExample", host, user);
    
    fast_log::init(Config::new().console()).unwrap();
    log::info!("Starting");

    log::info!("Migrate database structure");
    migrations::migrate(&mconn_string).await.unwrap();

    let data: Data<UsersService> = Data::new(UsersService::new(&rconn_string).await);
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(middleware::Logger::default())
            .service(list)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
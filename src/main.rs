#[macro_use]
extern crate rbatis;

use fast_log::config::Config;
use rbatis::rbatis::Rbatis;
use rbatis::crud::CRUD;
use rbatis::PageRequest;
use rbatis::Page;
use rbatis::wrapper::Wrapper;

use actix_web::{get, web, App, HttpServer, Responder};

use crate::entity::{Users};

mod migrations;
mod entity;

#[get("/hello")]
async fn list() -> impl Responder {
    format!("Hello!")
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

    let rb = Rbatis::new();

    rb.link(&rconn_string).await.unwrap();

    let req = PageRequest::new(1, 2);
    let wraper = Wrapper::new(&rbatis::DriverType::Postgres)
        .order_by(true, &["name"]);

    let users: Page<Users> = rb.fetch_page_by_wrapper(wraper,  &req).await.unwrap();
    log::info!("{}", serde_json::to_string(&users).unwrap());

    HttpServer::new(|| {
        App::new().service(list)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
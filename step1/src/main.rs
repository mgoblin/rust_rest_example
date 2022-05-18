#[macro_use]
extern crate rbatis;

use fast_log::config::Config;

use actix_web::{middleware, App, HttpServer};
use actix_web::web::{Data};

use crate::model::{Users};
use crate::service::UsersService;

mod migrations;
mod model;
mod service;
mod handlers;


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
            .service(handlers::list)
            .service(handlers::get_user_by_id)
            .service(handlers::create_user)
            .service(handlers::update_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
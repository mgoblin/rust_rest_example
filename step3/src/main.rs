use actix_web::{App, HttpServer, web::Data};
use services::UserInMemoryDAO;

mod model;
mod handlers;
mod http_utils;
mod services;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let user_dao = UserInMemoryDAO::new();
    let user_data = Data::new(user_dao); 
    HttpServer::new(move || {
        App::new()
            .app_data(user_data.clone())
            .service(handlers::list)
            .service(handlers::get_user_by_id)
            .service(handlers::create_user)
            .service(handlers::update_user)
            .service(handlers::delete_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
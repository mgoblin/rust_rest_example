use actix_web::{App, HttpServer, web::Data};
use configs::Configuration;
use services::UserInMemoryDAO;


mod model;
mod handlers;
mod http_utils;
mod services;
mod configs;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let cfg = &Configuration::load_from_file("application.yaml").unwrap();    
            
    let user_dao = UserInMemoryDAO::new(cfg);
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
    .bind((cfg.server.host.clone().as_str(), cfg.server.port))?
    .run()
    .await
}
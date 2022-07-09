use actix_web::{App, HttpServer, web::{Data, self}};
use configs::{Configuration, Store};
use services::{UserInMemoryDAO, UserDAO};


mod model;
mod handlers;
mod services;
mod configs;

fn create_dao(store: &Store) -> Box<dyn UserDAO + 'static> {
    Box::new(UserInMemoryDAO::new(store.inmemory.as_ref())) 
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let cfg = &Configuration::load_from_file("application.yaml").unwrap();    
    let store = cfg.store.as_ref().unwrap_or(&Store {inmemory: None});
    let dao = create_dao(store);

    let user_data = Data::new(dao); 

    HttpServer::new(move || {
        App::new()
            .app_data(user_data.clone())
            .route("/users", web::get().to(handlers::users_list))
            .service(handlers::get_user_by_id)
            .service(handlers::create_user)
            .service(handlers::update_user)
            .service(handlers::delete_user)
    })
    .bind((cfg.server.host.clone().as_str(), cfg.server.port))?
    .run()
    .await
}
use actix_web::{App, HttpServer, web::{Data, self}};
use configs::{Configuration, Store};
use services::{UserInMemoryDAO, UserDAO, UserDbDAO};


mod model;
mod handlers;
mod services;
mod configs;

fn create_dao(store: &Store) -> std::io::Result<Box<dyn UserDAO + 'static>> {
    match &store {
        &Store { inmemory: Some(im), db: None } => 
            Ok(Box::new(UserInMemoryDAO::new(Some(&im)))),
        &Store { inmemory: None, db: Some(dbcfg) } => 
            Ok(Box::new(UserDbDAO::new(&dbcfg))), 
        &Store { inmemory: None, db: None } => 
            Ok(Box::new(UserInMemoryDAO::new(store.inmemory.as_ref()))),
        &Store { inmemory: Some(_), db: Some(_) } => 
            Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Both inmemory and db config present")) 
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let cfg_result = &Configuration::load_from_file("./application.yaml");
    match cfg_result {
        Err(load_err) => {
            println!("Load config error {:#?}", &load_err);
            Ok(())
        },
        Ok(cfg) => {
            let store = cfg.store.as_ref().unwrap_or(&Store {inmemory: None, db: None});
            let dao = create_dao(store)?; 
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
    }
 }
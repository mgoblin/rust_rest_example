use actix_web::{App, HttpServer};

mod handlers;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(move || {
        App::new()
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
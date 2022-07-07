use actix_web::{Responder, web::{self, Data}, get, post, delete, HttpResponse};

use crate::{http_utils, services::{UserInMemoryDAO, UserDAO}, model::{User, UserName}};

pub async fn users_list(dao: Data<UserInMemoryDAO>) -> impl Responder {
    web::Json(dao.list())
}

#[get("users/{id}")]
pub async fn get_user_by_id(uid: web::Path<u64>, dao: Data<UserInMemoryDAO>) -> impl Responder {
    match dao.find_by_id(uid.into_inner()) {
        Some(u) => http_utils::user_as_json(&u),
        None => http_utils::user_not_found()
    }
}

#[post("users")]
pub async fn create_user(body_bytes: web::Bytes, dao: Data<UserInMemoryDAO>) -> impl Responder {
    let try_body = String::from_utf8(body_bytes.to_vec());
    match try_body {
        Ok(body) => {
            match dao.create(&UserName { name: body }) {
                Ok(u) => http_utils::user_as_json(&u),
                Err(e) => http_utils::user_not_modified(&e)
            }
        }, 
        Err(err) => HttpResponse::BadRequest().body(err.to_string())
    }
}

#[post("users/{id}")]
pub async fn update_user(uid: web::Path<u64>, body_bytes: web::Bytes, dao: Data<UserInMemoryDAO>) -> impl Responder {
    let try_body = String::from_utf8(body_bytes.to_vec());
    match try_body {
        Ok(body) => {
            let user = User {id: uid.into_inner(), user_name: UserName { name: body }};
            match dao.update(&user) {
                Ok(u) => http_utils::user_as_json(&u),
                Err(e) => http_utils::user_not_modified(&e)
            }
        },
        Err(err) => HttpResponse::BadRequest().body(err.to_string())
    }
}

#[delete("/users/{id}")]
pub async fn delete_user(uid: web::Path<u64>, dao: Data<UserInMemoryDAO>) -> impl Responder {
    match dao.delete_by_id(uid.into_inner()) {
        Ok(u) => http_utils::user_as_json(&u),
        Err(_) => http_utils::user_not_found()    
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::{test, http, App, body};
    use actix_web::web::{Data, Bytes};

    use crate::configs::{InMemory};
    use crate::services::UserInMemoryDAO;

    #[test]
    async fn test_list() {
        let dao = UserInMemoryDAO::new(Some(&InMemory {users: 0}));
        let result = users_list(Data::new(dao)).await;

        let req = &test::TestRequest::default().to_http_request();

        let resp = result.respond_to(req);
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_user_by_id_found() {
        let inmemory = InMemory {users: 1};

        let user_dao = UserInMemoryDAO::new(Some(&inmemory));
        let user_data = Data::new(user_dao); 

        let app = test::init_service(
            App::new()
                .app_data(user_data)
                .service(get_user_by_id),
        ).await;

        let req = test::TestRequest::get()
            .uri("/users/1")
            .to_request();
        let user: User = test::call_and_read_body_json(&app, req).await;

        assert_eq!(User { id: 1, user_name: UserName { name: "user 1".to_string() }}, user);
    }

    #[actix_web::test]
    async fn test_get_user_by_id_not_found() {
        let inmemory = InMemory {users: 0};

        let user_dao = UserInMemoryDAO::new(Some(&inmemory));
        let user_data = Data::new(user_dao); 

        let app = test::init_service(
            App::new()
                .app_data(user_data)
                .service(get_user_by_id),
        ).await;

        let req = test::TestRequest::get()
            .uri("/users/1")
            .to_request();
        let resp = test::call_service(&app, req).await;
        let status = resp.status();
        let body_box = resp.into_body();
        let body_bytes: Bytes = body::to_bytes(body_box).await.unwrap();
        

        assert_eq!(StatusCode::NOT_FOUND, status);
        assert_eq!(true, body_bytes.is_empty());
    }

    #[actix_web::test]
    async fn test_create_user() {
        let inmemory = InMemory {users: 0};

        let user_dao = UserInMemoryDAO::new(Some(&inmemory));
        let user_data = Data::new(user_dao); 

        let app = test::init_service(
            App::new()
                .app_data(user_data)
                .service(create_user),
        ).await;

        let req = test::TestRequest::post()
            .uri("/users")
            .set_payload("user 1")
            .to_request();
        let user: User = test::call_and_read_body_json(&app, req).await;

        assert_eq!(User {id: 1, user_name: UserName { name: "user 1".to_string() }}, user);
    }

    #[actix_web::test]
    async fn test_update_user() {
        let inmemory = InMemory {users: 1};

        let user_dao = UserInMemoryDAO::new(Some(&inmemory));
        let user_data = Data::new(user_dao); 

        let app = test::init_service(
            App::new()
                .app_data(user_data)
                .service(update_user),
        ).await;

        let req = test::TestRequest::post()
            .uri("/users/1")
            .set_payload("user 2")
            .to_request();
        let user: User = test::call_and_read_body_json(&app, req).await;

        assert_eq!(User {id: 1, user_name: UserName{ name: "user 2".to_string() }}, user);
    }

    #[actix_web::test]
    async fn test_delete_user() {
        let inmemory = InMemory {users: 1};

        let user_dao = UserInMemoryDAO::new(Some(&inmemory));
        let user_data = Data::new(user_dao); 

        let app = test::init_service(
            App::new()
                .app_data(user_data)
                .service(delete_user),
        ).await;

        let req = test::TestRequest::delete()
            .uri("/users/1")
            .to_request();
        let user: User = test::call_and_read_body_json(&app, req).await;

        assert_eq!(User {id: 1, user_name: UserName{ name: "user 1".to_string() }}, user);
    }
}


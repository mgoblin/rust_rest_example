use actix_web::{Responder, web::{self, Data}, get, post, delete};

use crate::{services::{UserInMemoryDAO, UserDAO}, model::{User, UserFields, UserDAOError}};

pub async fn users_list(dao: Data<UserInMemoryDAO>) -> Result<web::Json<Vec<User>>, UserDAOError> {
    dao.list().map(|list| web::Json(list))
}

#[get("users/{id}")]
pub async fn get_user_by_id(uid: web::Path<u64>, dao: Data<UserInMemoryDAO>) -> Result<web::Json<User>, UserDAOError> {
    dao.find_by_id(uid.into_inner()).map(|user| web::Json(user))
}

#[post("users")]
pub async fn create_user(fields: web::Json<UserFields>, dao: Data<UserInMemoryDAO>) -> Result<web::Json<User>, UserDAOError> {
    dao.create(&fields).map(|user| web::Json(user))
}

#[post("users/{id}")]
pub async fn update_user(uid: web::Path<u64>, fields: web::Json<UserFields>, dao: Data<UserInMemoryDAO>) -> impl Responder {
    let users_fields = fields.into_inner();
    let user = User {id: uid.into_inner(), fields: users_fields};
    dao.update(&user).map(|user| web::Json(user))
}

#[delete("/users/{id}")]
pub async fn delete_user(uid: web::Path<u64>, dao: Data<UserInMemoryDAO>) -> Result<web::Json<User>, UserDAOError> {
    dao.delete_by_id(uid.into_inner()).map(|user| web::Json(user))
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_web::{test, http, App, web::Data};

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

        assert_eq!(User { id: 1, fields: UserFields { name: "User1".to_string() }}, user);
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
        let resp: UserDAOError = test::call_and_read_body_json(&app, req).await;
        assert_eq!("User not found", resp.message);
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
            .insert_header(("Content-type", "application/json"))
            .set_payload("{ \"name\": \"User1\" }")
            .to_request();
        let user: User = test::call_and_read_body_json(&app, req).await;

        assert_eq!(User {id: 1, fields: UserFields { name: "User1".to_string() }}, user);
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
            .insert_header(("Content-type", "application/json"))
            .set_payload("{\"name\": \"User2\"}")
            .to_request();
        let user: User = test::call_and_read_body_json(&app, req).await;

        assert_eq!(User {id: 1, fields: UserFields{ name: "User2".to_string() }}, user);
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

        assert_eq!(User {id: 1, fields: UserFields{ name: "User1".to_string() }}, user);
    }
}


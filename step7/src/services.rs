use std::sync::Mutex;

use crate::configs::Db;
use crate::configs::InMemory;
use crate::model::DbUser;
use crate::model::User;
use crate::model::UserDAOError;
use crate::model::UserFields;
use actix_web::http::StatusCode;
use async_trait::async_trait;
use rbatis::crud::CRUD;
use rbatis::py_sql;
use rbatis::rbatis::Rbatis;
use rbatis::rb_py;
use validator::Validate;

#[async_trait]  
pub trait UserDAO: Sync + Send
{
    async fn list(&self) -> Result<Vec<User>, UserDAOError>;
    async fn find_by_id(&self, id: u64) -> Result<User, UserDAOError>;
    async fn create(&self, fields: &UserFields) -> Result<User, UserDAOError>;
    async fn update(&self, user: &User) -> Result<User, UserDAOError>;
    async fn delete_by_id(&self, id: u64) -> Result<User, UserDAOError>;
}


pub struct UserInMemoryDAO {
    users: Mutex<Vec<User>>,
}

impl UserInMemoryDAO {
    pub fn new(cfg: Option<&InMemory>) -> UserInMemoryDAO {
        
        let list = &mut vec![];
        
        if let Some(inmemory) = cfg {
            for i in 1 .. inmemory.users + 1 {
                let user = User {
                    id: u64::from(i), 
                    fields: UserFields { name: String::from(format!("User{}", i))}
                };
                list.push(user);
            }
        }
        UserInMemoryDAO{ users: Mutex::new(list.clone())} 
    }

    pub fn validate_fields(fields: &UserFields) -> Result<(), UserDAOError> {
        fields.validate()
            .map_err(|err| UserDAOError::from_validation_errors(&err))

    }
}

#[async_trait]
impl UserDAO for UserInMemoryDAO {
    
    async fn list(&self) -> Result<Vec<User>, UserDAOError> {
        let guard = self.users.lock().unwrap();
        let users = &*guard;
        let users_list = users.clone();
        Ok(users_list)
    }

    async fn find_by_id(&self, id: u64) -> Result<User, UserDAOError> {
        let guard = self.users.lock().unwrap();
        let users = &*guard;
        users.iter()
            .find(|&u| u.id == id)
            .map(|u| u.clone())
            .ok_or(UserDAOError {
                message: "User not found".to_string(), 
                status: StatusCode::NOT_FOUND.as_u16()
            })
    }

    async fn create(&self, fields: &UserFields) -> Result<User, UserDAOError> {
        
        UserInMemoryDAO::validate_fields(&fields)?;

        let mut guard = self.users.lock().unwrap();
        let users = &mut *guard;

        let user_exists = users.into_iter().any(|u| u.fields == *fields);

        if user_exists {
            Err(UserDAOError{ message: String::from("User exists"), status: StatusCode::BAD_REQUEST.as_u16()})
        } else {
            let max_id = users.into_iter().map(|u| u.id).max();
            let uid = max_id.unwrap_or(0) + 1;

            let user = User {id: uid, fields: fields.clone() };

            users.push(user.clone());

            Ok(user)
        }
    }

    async fn update(&self, user: &User) -> Result<User, UserDAOError> {
        
        UserInMemoryDAO::validate_fields(&user.fields)?;

        let mut guard = self.users.lock().unwrap();
        let users = &mut *guard;

        let existing_user_idx = users.into_iter().position(|u| u.id == user.id);

        match existing_user_idx {
            Some(idx) => {
                users.remove(idx);
                users.push(user.clone());
                Ok(user.clone())
            },
            None => Err(UserDAOError {message: String::from("User not found"), status: StatusCode::BAD_REQUEST.as_u16()})
        }
    }

    async fn delete_by_id(&self, id: u64) -> Result<User, UserDAOError> {
        let mut guard = self.users.lock().unwrap();
        let users = &mut *guard;

        let existing_user_idx = users.into_iter().position(|u| u.id == id);

        match existing_user_idx {
            Some(idx) => {
                let user = users.remove(idx);
                Ok(user.clone())
            },
            None => Err(UserDAOError {message: String::from("User not found"), status: StatusCode::BAD_REQUEST.as_u16()})
        }
    }
}

pub struct UserDbDAO {
    rb: Rbatis,
}

impl UserDbDAO {
    fn connection_str(cfg: &Db) -> String {
        format!("postgres://{2}:{3}@{0}:{4}/{1}", 
            cfg.host, 
            cfg.db_name,
            cfg.user,
            cfg.password,
            cfg.port
        )
    }

    pub async fn new(cfg: &Db) -> UserDbDAO {
        let rbatis = Rbatis::new();
        let conn_str = UserDbDAO::connection_str(cfg);

        rbatis.link(&conn_str).await.expect("rbatis not linked to db");

        UserDbDAO {
            rb: rbatis,
        }
    }

    #[py_sql("insert into users_schema.users(name) values ( #{uname} ) RETURNING id;")]
    async fn insert_with_identity(rb: &Rbatis, uname: &str) -> u64 { rbatis::impled!(); }

    #[py_sql("update users_schema.users set name = #{uuser.name} where id = #{uuser.id} RETURNING id;")]
    async fn update_by_id(rb: &Rbatis, uuser: &DbUser) -> u64 { rbatis::impled!(); }
}

#[async_trait]
impl UserDAO for UserDbDAO {
    async fn list(&self) -> Result<Vec<User>, UserDAOError> {
        let users = self.rb.fetch_list::<DbUser>().await;
        users
            .map(|db_users| {
                db_users.iter()
                    .map(|db_user | User { id: db_user.id, fields: UserFields {name: db_user.name.clone()} })
                    .collect()
            } )
            .map_err(|err| UserDAOError {status: 500, message: err.to_string()})
    }

    async fn find_by_id(&self, id: u64) -> Result<User, UserDAOError> {
        let user = self.rb.fetch_by_column::<DbUser, u64>("id", id).await;
        user
            .map (|db_user| User { id: db_user.id, fields: UserFields {name: db_user.name.clone()} })
            .map_err(|err| {
                match err {
                    rbatis::Error::E(_) => UserDAOError {status: 404, message: "User not found".to_string()},
                    rbatis::Error::Deserialize(msg) => UserDAOError {status: 500, message: msg.to_string()},
                    rbatis::Error::Database(msg) => UserDAOError {status: 500, message: msg.to_string()},
                    _ => UserDAOError {status: 500, message: "Unexpected error".to_string()},
                }
            })

    }

    async fn create(&self, fields: &UserFields) -> Result<User, UserDAOError> {
        
        UserInMemoryDAO::validate_fields(&fields)?;

        let uid: u64 = UserDbDAO::insert_with_identity(&self.rb, &fields.name)
            .await
            .map_err(|err: rbatis::Error| UserDAOError {status: 400, message: err.to_string()})?;

        Ok(User {id: uid, fields: fields.clone()})          
    }

    async fn update(&self, user: &User) -> Result<User, UserDAOError> {
        UserInMemoryDAO::validate_fields(&user.fields)?;

        let db_user = DbUser {id: user.id, name: user.fields.name.clone()};
        UserDbDAO::update_by_id(&self.rb, &db_user)
            .await
            .map_err(|err: rbatis::Error| UserDAOError {status: 400, message: err.to_string()})?;
        
        Ok(user.clone())
    }

    async fn delete_by_id(&self, id: u64) -> Result<User, UserDAOError> {
        let user = self.find_by_id(id).await?;
        self.rb.remove_by_column::<DbUser, u64>("id", id).await
            .map_err(|err: rbatis::Error| UserDAOError {status: 500, message: err.to_string()})?;
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use futures::executor::block_on;

    use crate::{configs::InMemory, model::{User, UserDAOError, UserFields}};

    use super::{UserInMemoryDAO, UserDAO};

    #[test]
    fn test_empty_list() {
        let dao = UserInMemoryDAO::new(None);
        let users = block_on(dao.list());
        assert_eq!(0, users.unwrap().len());
    }

    #[test]
    fn test_non_empty_list() {
        let dao = UserInMemoryDAO::new(Some(&InMemory { users: 1}));
        let users = block_on(dao.list());
        assert_eq!(users, Ok(vec![User { id: 1, fields: UserFields { name: "User1".to_string() }}]));
    }

    #[test]
    fn test_find_by_id_ok() {
        let dao = UserInMemoryDAO::new(Some(&InMemory { users: 2}));
        let user2 = block_on(dao.find_by_id(2));
        let expected = User{ id: 2, fields: UserFields { name: "User2".to_string() }};
        assert_eq!(Ok(expected), user2);  
    }

    #[test]
    fn test_find_by_id_not_found() {
        let dao = UserInMemoryDAO::new(Some(&InMemory { users: 2})); 
        let user5 = block_on(dao.find_by_id(5));
        assert_eq!(
            Err(
                UserDAOError { 
                    message: "User not found".to_string(), 
                    status: StatusCode::NOT_FOUND.as_u16() 
                }
            ), 
            user5);
    }

    #[test]
    fn test_find_by_id_not_found_on_empty_list() {
        let dao = UserInMemoryDAO::new(None); 
        let user5 = block_on(dao.find_by_id(1));
        assert_eq!(
            Err(UserDAOError { 
                message: "User not found".to_string(), 
                status: StatusCode::NOT_FOUND.as_u16()}
            ), 
            user5);
    }

    #[test]
    fn test_create_on_empty_list() {
        let dao = UserInMemoryDAO::new(None);
        let user = block_on(dao.create(&UserFields { name: "User".to_string() })).unwrap();
        let expected = User { id: 1, fields: UserFields{ name: "User".to_string() }};

        assert_eq!(expected, user);

        let user_in_list = block_on(dao.list()).unwrap().contains(&expected);
        assert_eq!(true, user_in_list);

        let finded_user = block_on(dao.find_by_id(expected.id));
        assert_eq!(Ok(expected) ,finded_user);
    }

    #[test]
    fn test_create_with_existing_name() {
        let dao = UserInMemoryDAO::new(Some(&InMemory {users: 1}));
        let result = block_on(dao.create(&UserFields { name: "User1".to_string() }));
        assert_eq!(Err(UserDAOError {message: "User exists".to_string(), status: StatusCode::BAD_REQUEST.as_u16()}), result);
    }

    #[test]
    fn test_create_with_empty_name() {
        let dao = UserInMemoryDAO::new(None);
        let result = block_on(dao.create(&UserFields { name: "".to_string() }));
        assert_eq!(Err(UserDAOError {
            message: "Validation failed for: field: 'name' errors: 'length, regex'".to_string(),
            status: StatusCode::BAD_REQUEST.as_u16()
        }), result);
    }

    #[test]
    fn test_update_existed() {
        let dao = UserInMemoryDAO::new(Some(&InMemory {users: 2}));
        let updated_user = User {id: 2, fields: UserFields { name: "Update_user".to_string() } };
        let user = block_on(dao.update(&updated_user)).unwrap();
        assert_eq!(updated_user, user);

        let finded_user2= block_on(dao.find_by_id(2)).unwrap();
        assert_eq!(updated_user, finded_user2);

        let finded_user1 = block_on(dao.find_by_id(1)).unwrap();
        assert_eq!(User {id: 1, fields: UserFields { name: "User1".to_string() }}, finded_user1);
    }

    #[test]
    fn test_update_non_existed() {
        let dao = UserInMemoryDAO::new(Some(&InMemory {users: 1}));
        let non_existed_user = User {id: 2, fields: UserFields { name: "Test".to_string() }};
        let result = block_on(dao.update(&non_existed_user)).unwrap_err();

        assert_eq!(UserDAOError { message: "User not found".to_string(), status: StatusCode::BAD_REQUEST.as_u16() }, result);

        let exists = block_on(dao.list()).unwrap().contains(&User {id: 1, fields: UserFields { name: "User1".to_string() }});
        assert_eq!(true, exists);
    }

    #[test]
    fn test_delete_existed() {
        let dao = UserInMemoryDAO::new(Some(&InMemory {users: 1}));
        assert_eq!(false, block_on(dao.list()).unwrap().is_empty());

        let expected_user = User {id: 1, fields: UserFields { name: "User1".to_string() }};
        let deleted_user = block_on(dao.delete_by_id(expected_user.id)).unwrap();
        
        assert_eq!(expected_user, deleted_user);

        assert_eq!(true, block_on(dao.list()).unwrap().is_empty());
    }

    #[test]
    fn test_delete_not_existed() {
        let dao = UserInMemoryDAO::new(Some(&InMemory {users: 0}));
        assert_eq!(true, block_on(dao.list()).unwrap().is_empty());

        let result = block_on(dao.delete_by_id(1)).unwrap_err();
        assert_eq!(UserDAOError {message: "User not found".to_string(), status: StatusCode::BAD_REQUEST.as_u16()}, result);

        assert_eq!(true, block_on(dao.list()).unwrap().is_empty());
    }

}


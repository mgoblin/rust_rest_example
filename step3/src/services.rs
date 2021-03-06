use std::sync::Mutex;

use crate::configs::InMemory;
use crate::model::User;
use crate::model::UserDAOError;

pub trait UserDAO {
    fn list(&self) -> Vec<User>;
    fn find_by_id(&self, id: u64) -> Option<User>;
    fn create(&self, name: &str) -> Result<User, UserDAOError>;
    fn update(&self, user: &User) -> Result<User, UserDAOError>;
    fn delete_by_id(&self, id: u64) -> Result<User, UserDAOError>;
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
                    name: String::from(format!("user {}", i))
                };
                list.push(user);
            }
        }
        UserInMemoryDAO{ users: Mutex::new(list.clone())} 
    }

    pub fn validate_name(name: &str) -> Result<&str, UserDAOError> {
        if name.trim().is_empty() {
            Err(UserDAOError{message: String::from("Empty name")})
        } else {
            Ok(name)
        }
    }
}

impl UserDAO for UserInMemoryDAO {
    
    fn list(&self) -> Vec<User> {
        let guard = self.users.lock().unwrap();
        let users = &*guard;
        users.clone()
    }

    fn find_by_id(&self, id: u64) -> Option<User> {
        let guard = self.users.lock().unwrap();
        let users = &*guard;
        users.into_iter().find(|u| u.id == id).cloned()
    }

    fn create(&self, name: &str) -> Result<User, UserDAOError> {
        
        UserInMemoryDAO::validate_name(&name)?;

        let mut guard = self.users.lock().unwrap();
        let users = &mut *guard;

        let user_exists = users.into_iter().any(|u| u.name == name.to_string());

        if user_exists {
            Err(UserDAOError{ message: String::from("User exists")})
        } else {
            let max_id = users.into_iter().map(|u| u.id).max();
            let uid = max_id.unwrap_or(0) + 1;

            let user = User {id: uid, name: name.to_string() };

            users.push(user.clone());

            Ok(user)
        }
    }

    fn update(&self, user: &User) -> Result<User, UserDAOError> {
        
        UserInMemoryDAO::validate_name(&user.name)?;

        let mut guard = self.users.lock().unwrap();
        let users = &mut *guard;

        let existing_user_idx = users.into_iter().position(|u| u.id == user.id);

        match existing_user_idx {
            Some(idx) => {
                users.remove(idx);
                users.push(user.clone());
                Ok(user.clone())
            },
            None => Err(UserDAOError {message: String::from("User not found")})
        }
    }

    fn delete_by_id(&self, id: u64) -> Result<User, UserDAOError> {
        let mut guard = self.users.lock().unwrap();
        let users = &mut *guard;

        let existing_user_idx = users.into_iter().position(|u| u.id == id);

        match existing_user_idx {
            Some(idx) => {
                let user = users.remove(idx);
                Ok(user.clone())
            },
            None => Err(UserDAOError {message: String::from("User not found")})
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{configs::InMemory, model::{User, UserDAOError}};

    use super::{UserInMemoryDAO, UserDAO};

    #[test]
    fn test_empty_list() {
        let dao = UserInMemoryDAO::new(None);
        let users = dao.list();
        assert_eq!(0, users.len());
    }

    #[test]
    fn test_non_empty_list() {
        let dao = UserInMemoryDAO::new(Some(&InMemory { users: 1}));
        let users = dao.list();
        assert_eq!(users, vec![User { id: 1, name: "user 1".to_string() }]);
    }

    #[test]
    fn test_find_by_id_ok() {
        let dao = UserInMemoryDAO::new(Some(&InMemory { users: 2}));
        let user2 = dao.find_by_id(2);
        let expected = User{ id: 2, name: "user 2".to_string() };
        assert_eq!(Some(expected), user2);  
    }

    #[test]
    fn test_find_by_id_not_found() {
        let dao = UserInMemoryDAO::new(Some(&InMemory { users: 2})); 
        let user5 = dao.find_by_id(5);
        assert_eq!(None, user5);
    }

    #[test]
    fn test_find_by_id_not_found_on_empty_list() {
        let dao = UserInMemoryDAO::new(None); 
        let user5 = dao.find_by_id(1);
        assert_eq!(None, user5);
    }

    #[test]
    fn test_create_on_empty_list() {
        let dao = UserInMemoryDAO::new(None);
        let user = dao.create("user").unwrap();
        let expected = User { id: 1, name: "user".to_string() };

        assert_eq!(expected, user);

        let user_in_list = dao.list().contains(&expected);
        assert_eq!(true, user_in_list);

        let finded_user = dao.find_by_id(expected.id);
        assert_eq!(Some(expected) ,finded_user);
    }

    #[test]
    fn test_create_with_existing_name() {
        let dao = UserInMemoryDAO::new(Some(&InMemory {users: 1}));
        let result = dao.create("user 1");
        assert_eq!(Err(UserDAOError {message: "User exists".to_string()}), result);
    }

    #[test]
    fn test_create_with_empty_name() {
        let dao = UserInMemoryDAO::new(None);
        let result = dao.create("");
        assert_eq!(Err(UserDAOError {message: "Empty name".to_string()}), result);
    }

    #[test]
    fn test_update_existed() {
        let dao = UserInMemoryDAO::new(Some(&InMemory {users: 2}));
        let updated_user = User {id: 2, name: "update user".to_string()};
        let user = dao.update(&updated_user).unwrap();
        assert_eq!(updated_user, user);

        let finded_user2= dao.find_by_id(2).unwrap();
        assert_eq!(updated_user, finded_user2);

        let finded_user1 = dao.find_by_id(1).unwrap();
        assert_eq!(User {id: 1, name: "user 1".to_string()}, finded_user1);
    }

    #[test]
    fn test_update_non_existed() {
        let dao = UserInMemoryDAO::new(Some(&InMemory {users: 1}));
        let non_existed_user = User {id: 2, name: "test".to_string() };
        let result = dao.update(&non_existed_user).unwrap_err();

        assert_eq!(UserDAOError { message: "User not found".to_string() }, result);

        let exists = dao.list().contains(&User {id: 1, name: "user 1".to_string()});
        assert_eq!(true, exists);
    }

    #[test]
    fn test_delete_existed() {
        let dao = UserInMemoryDAO::new(Some(&InMemory {users: 1}));
        assert_eq!(false, dao.list().is_empty());

        let expected_user = User {id: 1, name: "user 1".to_string()};
        let deleted_user = dao.delete_by_id(expected_user.id).unwrap();
        
        assert_eq!(expected_user, deleted_user);

        assert_eq!(true, dao.list().is_empty());
    }

    #[test]
    fn test_delete_not_existed() {
        let dao = UserInMemoryDAO::new(Some(&InMemory {users: 0}));
        assert_eq!(true, dao.list().is_empty());

        let result = dao.delete_by_id(1).unwrap_err();
        assert_eq!(UserDAOError {message: "User not found".to_string()}, result);

        assert_eq!(true, dao.list().is_empty());
    }

}


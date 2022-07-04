use std::sync::Mutex;

use crate::model::User;
use crate::model::UserDAOError;

pub trait UserDAO {
    fn list(&self) -> Vec<User>;
    fn find_by_id(&self, id: u64) -> Option<User>;
    fn create(&self, name: &str) -> Result<User, UserDAOError>;
    fn update(&self, user: &User) -> Result<User, UserDAOError>;
    fn delete(&self, id: u64) -> Result<User, UserDAOError>;
}

pub struct UserInMemoryDAO {
    users: Mutex<Vec<User>>,
}

impl UserInMemoryDAO {
    pub fn new() -> UserInMemoryDAO {
        let list = vec![
            User{id: 1, name: String::from("user 1")},
            User{id: 2, name: String::from("user 2")},
        ];
        UserInMemoryDAO{ users: Mutex::new(list)} 
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

    fn delete(&self, id: u64) -> Result<User, UserDAOError> {
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


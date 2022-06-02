use std::{error::Error, fmt::Display};
use crate::common::User;

#[derive(Debug)]
pub struct FindUserError {
    pub message: String,
}

impl Display for FindUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FindUserError(message: {})", self.message)    
    }
}

impl Error for FindUserError {}


#[derive(Debug)]
pub enum CommonError {
    Find(FindUserError),
    UserNotExists(String),
    AccessDenied
}


impl Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommonError::Find(ref e) => write!(f, "{}", e),
            CommonError::UserNotExists(s) => write!(f, "{}", s),
            CommonError::AccessDenied => write!(f, "Access denied"),
        }
    }
}

impl Error for CommonError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CommonError::Find(ref e) => Some(e),
            CommonError::UserNotExists(_) => None,
            CommonError::AccessDenied => None
        }
    }
}

pub fn e_find_user(uid: u64) -> Result<Option<User>, FindUserError> {
    if uid < 10 {
        Err(FindUserError {message: format!("Find user error. User id {}", uid)})
    } else if (10..20).contains(&uid) {
        let uname = format!("user name {}", uid);
        let user = User::new(uid, &uname[..]);
        Ok(Some(user))
    } else {
        Ok(None)
    }
}

pub fn e_find_user2(uid: u64) -> Result<User, CommonError> {
    e_find_user(uid)
    .map_err(CommonError::Find)?
    .ok_or_else(|| CommonError::UserNotExists(format!("User with id {} not found", uid)))
    .and_then(|user| 
        if user.id() == 11 {
            Err(CommonError::AccessDenied)
        } else {
            Ok(user)
        }  
    )   
 }

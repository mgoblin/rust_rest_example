use crate::common::User;

#[derive(Debug)]
pub struct FindUserError {
    pub message: String,
}

#[derive(Debug)]
pub struct CommonError {
    pub message: String,
}

pub fn find_user(uid: u64) -> Result<Option<User>, FindUserError> {
    if uid < 10 {
        Err(FindUserError {message: format!("Find user error. User id {}", uid)})
    } else if (10..20).contains(&uid) {
        let uname = format!("user name {}", uid);
        let user = User::new(uid, uname.as_str());
        Ok(Some(user))
    } else {
        Ok(None)
    }
}

pub fn find_user3(uid: u64) -> Result<User, CommonError> {
    find_user(uid)
    .map_err(|e| CommonError {message: e.message})?
    .ok_or(CommonError {message: String::from("User not found")})
    .and_then(|user| 
        if user.id() == 11 {
            Err(CommonError { message: String::from("Access denied")})
        } else {
            Ok(user)
        }  
    )    
 }

 pub fn find_user2(uid: u64) -> Result<User, CommonError> {
    let r = find_user(uid);
    if let Ok(Some(user)) = r {
        if user.id() == 11 {
            let err = CommonError { message: String::from("Access denied")};
            Err(err) 
        } else {
            Ok(user)
        }
    } else if let Ok(None) = r {
        let err = CommonError { message: String::from("User not found")};
        Err(err)
    } else {
        let err = CommonError { message: r.unwrap_err().message};
        Err(err)
    }    
 }


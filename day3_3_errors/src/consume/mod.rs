
#[derive(Debug)]
pub struct User {
    id: u64,
    name: String
} 

impl User {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

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
    } else if uid >= 10 && uid < 20 {
        let uname = format!("user name {}", uid);
        let user = User {id: uid, name: uname};
        Ok(Some(user))
    } else {
        Ok(None)
    }
}

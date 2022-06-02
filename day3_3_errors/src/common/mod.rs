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

    pub fn new(uid: u64, uname: &str) -> User {
        User {id: uid, name: String::from(uname)}
    }
}

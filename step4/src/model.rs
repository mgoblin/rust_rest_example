use serde::{Serialize, Deserialize};
use std::{error::Error, fmt::Display};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
  pub id: u64,
  
  #[serde(flatten)]
  pub user_name: UserName,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserName {
  pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserDAOError {
  pub message: String,
}

impl Display for UserDAOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UserDAO error {}", self.message)
    }
}

impl Error for UserDAOError {}

#[cfg(test)]
mod tests {
    use crate::model::UserName;

    use super::User;


  #[test]
  fn test_serialize_user() {
    let json = serde_json::to_string(&User {
      id: 1, 
      user_name: UserName { name: "user".to_string()}
    }).unwrap();
    assert_eq!("{\"id\":1,\"name\":\"user\"}", json);
  }

  #[test]
  fn test_serizalize_users_list() {
    let users = vec![
      User{id: 1, user_name: UserName { name: "user 1".to_string()}},
      User{id: 2, user_name: UserName { name: "user 2".to_string()}},
    ];

    let json = serde_json::to_string(&users).unwrap();
    assert_eq!("[{\"id\":1,\"name\":\"user 1\"},{\"id\":2,\"name\":\"user 2\"}]", json);
  }

  #[test]
  fn test_deserialize_user() {
    let json = "{\"id\":1,\"name\":\"user\"}";
    let user = serde_json::from_str::<User>(json).unwrap();
    let expected_user = User {
      id: 1, 
      user_name: UserName { name: "user".to_string()}};

    assert_eq!(expected_user, user);
  }

  #[test]
  fn test_deserialize_users_list() {
    let json ="[{\"id\":1,\"name\":\"user 1\"},{\"id\":2,\"name\":\"user 2\"}]";
    let users = serde_json::from_str::<Vec<User>>(json).unwrap();
    assert_eq!(vec![
        User {id: 1, user_name: UserName { name: "user 1".to_string() }},
        User {id: 2, user_name: UserName { name: "user 2".to_string() }},
      ], 
      users
    );
  }

}

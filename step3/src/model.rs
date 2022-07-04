use serde::{Serialize, Deserialize};
use std::{error::Error, fmt::Display};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
  pub id: u64,
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

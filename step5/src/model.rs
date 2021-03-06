use actix_web::{ResponseError, HttpResponse, http::StatusCode};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationErrors, ValidationErrorsKind, ValidationError};
use std::{error::Error, fmt::Display};

lazy_static! {
  static ref STARTS_WITH_UPPER_LETTER: Regex = Regex::new(r"^[A-Z][a-zA-Z\d_]+$").unwrap();
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
  pub id: u64,
  
  #[serde(flatten)]
  pub fields: UserFields,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct UserFields {
  #[validate(length(min = 4, max = 255), non_control_character, regex = "STARTS_WITH_UPPER_LETTER")]
  pub name: String
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserDAOError {
  #[serde(rename(serialize = "error", deserialize = "error"))]
  pub message: String,
  #[serde(skip)]
  pub status: u16
}

impl UserDAOError {
  pub fn from_validation_errors(err: &ValidationErrors) -> UserDAOError {
    
    fn element_type(err: &ValidationErrorsKind) -> String {
      match err {
        ValidationErrorsKind::Struct(_) => String::from("struct"),
        ValidationErrorsKind::List(_) => String::from("list"),
        ValidationErrorsKind::Field(_) => String::from("field"),
      }
    }

    fn str(err: &ValidationErrorsKind) -> String {
        match err {
          ValidationErrorsKind::Struct(v) => format!("{:?}", v),
          ValidationErrorsKind::List(v) => format!("{:?}", v),
          ValidationErrorsKind::Field(f) => field_errors_str(f),
        }
    }

    fn field_errors_str(f: &Vec<ValidationError>) -> String {
      let errs_txt: Vec<&str> = f.iter()
        .map(|e| e.code.as_ref())
        .collect();

      errs_txt.join(", ")
    }

    let error_vals: String = err.errors()
      .iter()
      .map(|(f, err)|  format!("{}: '{}' errors: '{}'", element_type(err), f, str(err)))
      .collect::<Vec<String>>()
      .join("# ");
      
    UserDAOError {message: format!("Validation failed for: {}", error_vals), status: StatusCode::BAD_REQUEST.as_u16() }
  }
}

impl Display for UserDAOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UserDAO error {}", self.message)
    }
}

impl Error for UserDAOError {}

impl ResponseError for UserDAOError {
  
  fn error_response(&self) -> HttpResponse {
    let err_json = serde_json::json!({ "error": self.message });
    HttpResponse::build(StatusCode::from_u16(self.status).unwrap()).json(err_json)
  }
}


#[cfg(test)]
mod tests {
    use crate::model::UserFields;

    use super::User;


  #[test]
  fn test_serialize_user() {
    let json = serde_json::to_string(&User {
      id: 1, 
      fields: UserFields { name: "user".to_string()}
    }).unwrap();
    assert_eq!("{\"id\":1,\"name\":\"user\"}", json);
  }

  #[test]
  fn test_serizalize_users_list() {
    let users = vec![
      User{id: 1, fields: UserFields { name: "user 1".to_string()}},
      User{id: 2, fields: UserFields { name: "user 2".to_string()}},
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
      fields: UserFields { name: "user".to_string()}};

    assert_eq!(expected_user, user);
  }

  #[test]
  fn test_deserialize_users_list() {
    let json ="[{\"id\":1,\"name\":\"user 1\"},{\"id\":2,\"name\":\"user 2\"}]";
    let users = serde_json::from_str::<Vec<User>>(json).unwrap();
    assert_eq!(vec![
        User {id: 1, fields: UserFields { name: "user 1".to_string() }},
        User {id: 2, fields: UserFields { name: "user 2".to_string() }},
      ], 
      users
    );
  }

}

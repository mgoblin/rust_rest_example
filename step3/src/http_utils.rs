use actix_web::{HttpResponse, http::{StatusCode, header::ContentType}};

use crate::model::{User, UserDAOError};

pub fn user_as_json(user: &User) -> HttpResponse {
    let user_str = serde_json::to_string_pretty(user);
    match user_str {
      Ok(s) => 
        HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::json())
            .body(s),
      Err(err) =>  HttpResponse::from_error(err)
    } 
}

pub fn user_not_found() -> HttpResponse {
  HttpResponse::build(StatusCode::NOT_FOUND)
    .content_type(ContentType::json())
    .finish()
}

pub fn user_not_modified(e: &UserDAOError) -> HttpResponse {
  let err_str = serde_json::to_string_pretty(e);

  match err_str {
    Ok(s) => 
      HttpResponse::build(StatusCode::BAD_REQUEST)
        .content_type(ContentType::json())
        .body(s),
    Err(err) => HttpResponse::from_error(err)                  
  }
}
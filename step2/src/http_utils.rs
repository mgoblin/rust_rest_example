use actix_web::{HttpResponse, http::{StatusCode, header::ContentType}};

use crate::model::User;

pub fn user_as_json(user: &User) -> HttpResponse {
    let user_str = serde_json::to_string(user);
    match user_str {
      Ok(s) => 
        HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::json())
            .body(s),
      Err(err) =>  HttpResponse::from_error(err)
    } 
}
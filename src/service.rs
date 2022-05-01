use rbatis::rbatis::Rbatis;
use rbatis::{Page, PageRequest};
use rbatis::wrapper::Wrapper;
use rbatis::crud::CRUD;

use crate::Users;

pub struct UsersService {
    rb: Rbatis
}

impl UsersService {
    pub async fn new(conn_str: &str) -> UsersService {
        let rbatis = Rbatis::new();

        rbatis.link(conn_str).await.expect("rbatis not linked to db");

        UsersService{
            rb: rbatis,
        }
    }

    pub async fn list(&self, req: &PageRequest) -> Page<Users> {
        let wraper = Wrapper::new(&rbatis::DriverType::Postgres)
            .order_by(true, &["name"]);

        self.rb.fetch_page_by_wrapper(wraper,  req).await.unwrap()
    }

    pub async fn find_by_id(&self, id: u64) -> std::option::Option<Users> {
        self.rb.fetch_by_column("id", id).await.unwrap()
    }
}
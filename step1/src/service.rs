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
        let wrapper = Wrapper::new(&rbatis::DriverType::Postgres)
            .order_by(true, &["name"]);

        self.rb.fetch_page_by_wrapper(wrapper,  req).await.unwrap()
    }

    pub async fn find_by_id(&self, id: u64) -> std::option::Option<Users> {
        self.rb.fetch_by_column("id", id).await.unwrap()
    }

    pub async fn create_user(&self, uname: &str) -> Users {
        let uid = UsersService::insert_with_identity(&self.rb, uname).await.unwrap();
        Users {
            id: uid,
            name: uname.to_string(),
        }
    }

    pub async fn update_user(&self, uuser: &Users) -> Option<Users> {
        log::info!("update_user {:?}", uuser);
        let result = UsersService::update_with_identity(&self.rb, uuser).await.unwrap();
        log::info!("update result = {:?}", result);
        match result {
            Some(uid) => Some(Users{id: uid, name: uuser.name.clone()}),
            None => None
        }
    }

    #[py_sql("insert into users(name) values (#{uname}) RETURNING id;")]
    async fn insert_with_identity(rb: &Rbatis, uname: &str) -> u64 { impled!() }

    #[py_sql("update users set name = #{uuser.name} where id = #{uuser.id} RETURNING id;")]
    async fn update_with_identity(rb: &Rbatis, uuser: &Users) -> Option<u64> { impled!() }
}
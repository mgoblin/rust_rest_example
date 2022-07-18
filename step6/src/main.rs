use tokio_postgres::{NoTls, Error};

use crate::model::{User, UserFields};

mod model;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost dbname=users user=rw_user password=123qweasd", NoTls).await?;    

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client
        .query("SELECT * FROM users_schema.users u", &[])
        .await?;
    
    rows.into_iter().for_each(|r| { 
        let uid: i64 = r.get(0);
        let uname: &str = r.get(1);
        let user = User {id: uid as u64, fields: UserFields { name: uname.to_string() } };
        println!("user = {:#?}", user); 
    });    
    
    Ok(())
}
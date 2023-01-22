use std::{time::{self}, collections::HashMap, env};


use account::config::AccountConfig;
use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime};

use rocket::{figment::providers::{ Env, Toml}, serde::Deserialize, routes};
use tokio_postgres::NoTls;

use crate::account::config::Account;

mod account;
mod session;

//api/account/new POST
//api/account/remove POST
//api/account/update POST   

//api/thread/new POST 
//api/thread/remove POST
//api/thread/update POST
//api/thread/retrieve POST 

#[rocket::main]
async fn main() -> Result<(), rocket::Error> { 

    // Postgres Database
    let pg_dbname = Some(
    dotenv::var("PG_DBNAME")
    .expect("PG_DBNAME NOT SET"));
    let pg_user = Some(
    dotenv::var("PG_USER")
    .expect("PG_USER NOT SET"));
    let pg_pass = Some(
    dotenv::var("PG_PASS")
    .expect("PG_PASS NOT SET"));
    let pg_port: Option<u16> = Some(
    dotenv::var("PG_PORT")
    .expect("PG_PORT NOT SET")
    .parse().unwrap());
    let mut pg_cfg = Config::new();
    pg_cfg.dbname = pg_dbname;
    pg_cfg.user = pg_user;
    pg_cfg.password = pg_pass;
    pg_cfg.port = pg_port;
    pg_cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });

    let pool = pg_cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    let _rocket = rocket::build()
    .mount("/api", account::routes::routes()).manage(pool)
        .ignite().await?
        .launch().await?;
    //    let acc_cfg = AccountConfig::new("accounts", pool);

    //let acc = acc_config.find("16743445648412869600").await.unwrap();
    //let acc = Account::new("piledriver", "ddas", "piledriver@gmail.com");  
    //acc_config.create(acc).await.unwrap();

    //let acc = acc_config.find("1673919920888240800").await.unwrap();

    /* 
    let acc_config = AccountConfig::new("accounts", pool);
    let acc = Account::new("poddd", "iloveyoud", "zedljdkod@gmail.com");
    acc_config.create(acc).await.unwrap();
    */

    // Creates session with the user id...
   // Session::with("1673919920888240800").create_session();
    
    //AccountConfig
    
    //let acc_config = user::AccountConfig::new(pool);
    //let acc = user::Account::new("john12", "12346", "hoar@gmail.com");  
    //acc_config.create_account(acc).await.unwrap();
    //let yes = acc_config.account_exists("id", "1673746031502691400").await;
    
   // let manage = acc_config.create_acc(acc)


    
    
    
    /*
        let acc_config = user::Config::new();
        acc_config.set_pg_db(Acc);
        let acc = acc_config.load_acc(id_representation);
        acc_config = user::Session::load(){}
    */
    
    
    //let acc = Account::new("johndoed", "1234", "johndoe@gmail.com");
   // let acc = Account::load_account
    //let acc_m = acc.m(&client_object);
    //acc_m.update(AccountField::Rank, Rank::Admin).await.unwrap();

    
   // acc_m.exists(AccountField::Email).await.unwrap();

    /*      
    let yes = Postgres::new();

    let test = 
    AccountManager::new(client, 
        Account::new( "3".to_string(),
        "dog".to_string(), 
        "123467".to_string(), 
        "dog@gmail.com".to_string(), 
        "member".to_string()\
    ));
    test.await.update(AccountField::Username, "doggy").await.unwrap();  

      id: String,
    username: String,
    password: String,
    email: String,
    role: String,
    */
    
    /* 
    let test = ureq::post("http://127.0.0.1:8000/ad")
    .send_json(ureq::json!({
        "id": time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos().to_string(),
        "username": "boss",
        "password": "12345",
        "email": "boss@gmail.com",
        "role": "Admin",
    })).unwrap();
    */

    /* 
    let test = 
    AccountManager::new(&client, 
        Account::new( "das".to_string(),
        "dassda".to_string(), 
        "123467".to_string(), 
        "dog@das.com".to_string(), 
        "member".to_string()
    ));
    test.create().await.unwrap();
*/

    Ok(())
}
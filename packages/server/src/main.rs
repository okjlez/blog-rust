use std::{time::{self}, collections::HashMap, env};


use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime};

use rocket::{figment::providers::{ Env, Toml}, serde::Deserialize, routes};
use tokio_postgres::NoTls;
use std::time::UNIX_EPOCH;

mod models;
mod session;
mod error;

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
    let client_object = pool.get().await.unwrap();

    
    /* 
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

    let _rocket = rocket::build()
        .mount("/api", routes![]).manage(client_object)
            .ignite().await?
            .launch().await?;

            /* 
    let mut map = HashMap::new();
    map.insert("id", time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos().to_string());
    map.insert("username", "doggDDy".to_string());
    map.insert("password", "dasdsa".to_string());
    map.insert("email", "boss444@gmail.com".to_string());
    map.insert("role", "member".to_string());
    
    let respon = r_client.post("http://127.0.0.1:8000/api/account/new")
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
        .unwrap();

        println!("{:#?}", respon);
        */

    /* 
            let test = ureq::post("http://127.0.0.1:8000/api/account/new")
            .set("Content-Type", "application/json")
            .set("Content-Length", "1024")
            .set("Host", "127.0.0.1:8000")
            .send_json(ureq::json!({
                "id": time::SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos().to_string(),
                "username": "444",
                "password": "4444",
                "email": "boss444@gmail.com",
                "role": "Ad44min",
            })).unwrap();
        
            println!("{:#?}", test.status());
            */
    Ok(())
}
/// A simple blog system written in rust. I made the restapi with basic 
/// authentication(cookie auth). 
/// 
/// Please make sure the secure, and httponly flags are enabled. 
/// 
/// My first actual project in Rust.
/// 
/// * what it does not support
/// multiple devices...

/// REST API REQUESTS
/// * ACCOUNTS * 
/// /api/account/new POST
/// /api/account/update POST   

/// * THREADS * 
/// /api/thread/new POST 
/// /api/thread/remove POST
/// /api/thread/update POST
/// /api/thread/retrieve POST 
/// /api/thread/{name}


use account::{config::AccountConfig};
use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime};

use rocket::{figment::providers::{ Env, Toml}, serde::Deserialize, routes};
use tokio_postgres::NoTls;

mod account;
mod session;
mod thread;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> { 

    // Postgres Database
    let pg_dbname = Some(dotenv::var("PG_DBNAME").expect("PG_DBNAME NOT SET"));
    let pg_user = Some(dotenv::var("PG_USER").expect("PG_USER NOT SET"));
    let pg_pass = Some(dotenv::var("PG_PASS").expect("PG_PASS NOT SET"));let pg_port: Option<u16> = Some(dotenv::var("PG_PORT").expect("PG_PORT NOT SET").parse().unwrap());
    let mut pg_cfg = Config::new();
    pg_cfg.dbname = pg_dbname;
    pg_cfg.user = pg_user;
    pg_cfg.password = pg_pass;
    pg_cfg.port = pg_port;
    pg_cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });

    let pool = pg_cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    let acc_cfg = AccountConfig::new(&pool);

    let _rocket = rocket::build()
    .mount("/api", account::routes::routes())
    .mount("/api", thread::routes::routes()).manage(pool)
        .ignite().await?
        .launch().await?;
    Ok(())
}
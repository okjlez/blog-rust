use std::time::SystemTime;

use deadpool_postgres::Pool;
use postgres_types::{FromSql, ToSql};
use rocket::{serde::{Serialize, Deserialize}, tokio::time, FromForm};
use std::time::UNIX_EPOCH;

use crate::account::config::AccountConfig;


pub struct ThreadManager {
    thread: Thread
}

impl ThreadManager {
    pub async fn save(&self, cfg: AccountConfig<'_>)  {
        let sql = "select create_thread($1, $2, $3, $4)";
        let query = cfg.quik_query(sql, &[&self.thread.title, &self.thread.body, &self.thread.created_by, &self.thread.created_on]).await;
        match query {
            Ok(_) => {
                println!("[Thread] {} created a post with name {} ", &self.thread.created_by, &self.thread.title());
            },
            Err(er) => {
                println!("[Thread] {} failed to create a post err: {:#?} ", &self.thread.created_by, er.as_db_error());
            },
        }
    }
}

#[derive(Debug, FromForm, ToSql, FromSql)]
pub struct Thread {
    title: String,
    body: String,
    #[field(default = "")] 
    created_by: String,
    #[field(default = "")]
    created_on: String,
}

impl Thread {
    pub fn new(
        title: &str, 
        body: &str, 
        created_by: &str,
    ) -> ThreadManager {
        let mut default = Thread::default();
        default.title = title.to_string();
        default.body = body.to_string();
        default.created_by = created_by.to_string();
        ThreadManager { thread: default }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn body(&self) -> &String {
        &self.title
    }

    pub fn created_on(&self) -> &String {
        &self.title
    }

    pub fn created_by(&self) -> &String {
        &self.title
    }
}


impl Default for Thread {
    fn default() -> Self {
        Self { 
            title: Default::default(), 
            body: Default::default(), 
            created_by: "".to_string(), 
            created_on: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string()
        }
    }
}
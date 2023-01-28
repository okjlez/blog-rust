use std::time::{SystemTime, UNIX_EPOCH};

use nanoid::nanoid;
use postgres_types::ToSql;
use rocket::{serde::{Serialize, Deserialize}};

use crate::account::config::AccountConfig;

#[derive(Debug, Deserialize, Serialize, ToSql)]
#[serde(crate = "rocket::serde")]
pub struct Session {
    pub session_id: String,
    pub account_id: String,
    pub expires_in: String
}

impl Session {
    pub fn new(account_id: &str) -> Session {
        let mut session = Session::default();
        session.account_id = account_id.to_string();
        session
    }

    pub async fn save(&self, cfg: AccountConfig<'_>) {
        let sql = "SELECT create_session($1, $2, $3)";
        // rushed...
        let query = cfg.quik_query(sql, & [&self.session_id, &self.account_id, &self.expires_in]).await;
        match query {
            Ok(suc) => {
                println!("Successfully created sesison")
            },
            Err(er) => {
                println!("{:#?}", er.as_db_error());
            },
        }
    }
}

impl Default for Session {
    fn default() -> Self {
        let created_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        Self { 
            session_id: nanoid!(), 
            account_id: Default::default(),
            expires_in: ((created_at + 604800000)).to_string() // one week from now
        }
    }
}
use nanoid::nanoid;
use rocket::{http::{Cookie, SameSite}, time::{Duration, OffsetDateTime}, serde::{Serialize, Deserialize}};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Session {
    pub session_id: String,
    pub account_id: String,
}

impl Session {
    pub fn new(account_id: &str) -> Session {
        let mut session = Session::default();
        session.account_id = account_id.to_string();
        session
    }
}

impl Default for Session {
    fn default() -> Self {
        Self { 
            session_id: nanoid!(), 
            account_id: Default::default()
        }
    }
}
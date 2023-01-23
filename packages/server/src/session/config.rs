use nanoid::nanoid;
use rocket::{http::{Cookie, SameSite}, time::{Duration, OffsetDateTime}};

pub struct Session {
    session_id: String,
    account_id: String,
}

impl Session {
    pub fn new(account_id: &str) -> Session {
        let mut session = Session::default();
        session.account_id = account_id.to_string();
        session
    }
    
    fn cookie(&self) -> Cookie {
        let mut now = OffsetDateTime::now_utc();
        now += Duration::seconds(604800); // 1 week
        Cookie::build("sid", &self.session_id)
        .same_site(SameSite::Lax)
        .expires(now)
        .finish()
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
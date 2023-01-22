
pub struct Session {
    session_id: String,
    account_id: String
}

impl Session {
    pub fn new(session_id: &str, account_id: &str) -> Session {
        Session { 
            session_id: session_id.to_string(), 
            account_id: account_id.to_string()
        }
    }
}
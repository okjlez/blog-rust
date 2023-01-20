
/// A struct that will manage the user's session via cookies.
/// hence while we have 'name' and 'expires_in' inside the struct.
struct SessionManager;

impl SessionManager {
    pub fn create_session(){}
}

struct Session {
    name: String,
    session_id: String,
    account_id: String,
    expires_in: u64
}
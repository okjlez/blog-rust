use nanoid::nanoid;

use crate::account::config::Account;

/// A struct that will manage the user's session hence why.
/// we have 'name' and 'expires_in' inside the struct.
pub struct SessionManager {
    session: Session
}

impl SessionManager {
    pub fn start(&self){}
}

/// The blueprint for the session.
/// 
/// Adjust the data accordingly to reflect a user's Account.
/// 
/// 'Name' represents the cookie's name should be the same
/// for every individual. In order to avoid problems trying
/// to dynamically retrieve data from the cookie.
/// 
/// 'Expires_in' is how long you want your Cookie to last.
pub struct Session {
    name: String,
    session_id: String,
    account_id: String,
    expires_in: u64
}

impl Session {
    /// Constructs a [`Session`]. Returns the [`SessionManager`]
    /// Necessary because SessionManager will create the cookie
    /// for us.
    /// 
    /// # Example
    ///
    /// create an original session (not simplified).
    /// 
    /// ```rust
    /// use account::session::Session;
    ///
    /// let session = Session::new(
    /// "sid", // name of cookie
    /// "Yo1Tr9F3iF-LFHX9i9GvA", // session_id should be unique
    /// "1673919920888240800",  // the id of the account
    ///  604800); // how long should the session last for.
    ///  
    /// session.create_session() // creates the session(db insertion)
    /// ```
    pub fn new(
        name: &str, 
        sess_id: &str, 
        acc_id: &str, 
        expires_in: u64
    ) -> SessionManager {
         let session = Session {
            name: name.to_string(),
            session_id: sess_id.to_string(),
            account_id: acc_id.to_string(),
            expires_in
         };
         SessionManager { session }
    }

    /// Simplified version of [`Session`] the session_id,
    /// expiration date, and name are automated. 
    /// Takes [`Session`] as input.
    ///
    /// # Example
    ///
    /// Create a simplified version of Session::new()
    /// 
    /// ```rust
    /// use account::session::Session;
    /// let acc = Account::new("zeljko", "iloveyou", "zeljko@gmail.com");
    /// //TODO: FINISH THIS!
    /// let session = Session::with("1673919920888240800")
    /// session.create_session();
    /// ```
    pub fn with(acc: Account) -> SessionManager {
        let mut sess = Session::default();
        sess.account_id = acc.id().to_string();
        SessionManager { session: sess }
    }

    /// Create a session based on the account_id.
    ///
    /// # Example
    ///
    /// Create a simplified version of Session::new()
    /// 
    /// ```rust
    /// use account::session::Session;
    ///
    /// let session = Session::with_id("1673919920888240800")
    /// ```
    pub fn with_id(acc_id: &str) -> SessionManager {
        let mut sess = Session::default();
        sess.account_id = acc_id.to_string();
        SessionManager { session: sess }
    }
}

impl Default for Session {
    fn default() -> Self {
        Self { 
            name: "sid".to_string(), // default name of the session.
            session_id: nanoid!(), 
            account_id: Default::default(), 
            expires_in: 604800 // 1 week in seconds...
        }
    }
}
use core::fmt;


#[derive(Clone, PartialEq, Debug)]
pub enum Error {
    AccountNotFound(String),
    SessionNotFound,
    AccountExists(String),
    UsernameTaken(String),
    EmailTaken(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            // TODO: Add error messages
            Error::AccountNotFound(account) => write!(
                f,
                "The account '{}' was not found.",
                account
            ),
            Error::SessionNotFound => write!(
                f,
                "Unable to locate the session of the cookie. Does it exist?"
            ),
            Error::AccountExists(account) => write!(
                f,
                "This account '{}' already exists" ,
                account
            ),
            Error::UsernameTaken(username) => write!(
                f,
                "The username '{}' is already taken",
                username
            ),
            Error::EmailTaken(email) => write!(
                f,
                "The email '{}' is already taken",
                email
            )
        }
    }
}
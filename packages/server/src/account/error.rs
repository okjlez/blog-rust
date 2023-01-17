use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum AccountError {
    UsernameTaken(String),
    EmailTaken(String),
    InvalidFormat(String)
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            AccountError::UsernameTaken(username) => write!(
                f,
                "The username {} is taken.",
                username
            ),
            AccountError::EmailTaken(email) => write!(
                f,
                "The email '{}' was not found.",
                email
            ),
            AccountError::InvalidFormat(db_error_message) => write!(
                f,
                "{}",
                db_error_message
            ),
        }
    }
}
use std::fmt;

use tokio_postgres::error::DbError;

use super::config::Account;

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

impl AccountError {
    pub fn parse_db_error(er: Option<&DbError>, acc: &Account, message: String) -> AccountError {
        let error = er.unwrap();
        let code = error.code().code().to_string();

        // find a better way.
        if code.eq("42P10") {
            return AccountError::UsernameTaken(acc.username().to_string())
        }
        if code.eq("42P11") {
            return AccountError::EmailTaken(acc.email().to_string())
        }
        return AccountError::InvalidFormat(message)
    }
}
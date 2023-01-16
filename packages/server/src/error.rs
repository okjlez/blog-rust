use core::fmt;


#[derive(Clone, PartialEq, Debug)]
pub enum Error {
    //models//user.rs 
    AccountNotFound(String),
    AccountExists(String),
    UniqueViolation(String),
    ConstraintViolation(String),
    IdNotMutable(String),
    WrongDataType(String, String),
    //models//sesison.rs
    SessionNotFound,

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
            Error::IdNotMutable(id) => write!(
                f,
                "Id change is not permittable {}",
                id
            ),
            Error::WrongDataType(need, given) => write!(
                f,
                "Wrong datatype NEED({}) != GIVEN({})",
                need, given
            ),
            Error::UniqueViolation(db_message) => write!(
                f,
                "{}", 
                db_message
            ),
            Error::ConstraintViolation(db_message) => write!(
                f,
                "{}", 
                db_message
            ),
        }
    }
}
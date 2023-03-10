use core::fmt;

use postgres_types::{ToSql, FromSql};
use rocket::serde::{Serialize, Deserialize};

#[derive(
    Default, Debug, Clone, PartialEq, 
    Serialize, Deserialize, 
    ToSql, FromSql
)]
#[serde(crate = "rocket::serde")]
pub enum Rank {
    None, // if you want to add sms or smtp verification this will be of use to you.
    #[default]
    Member,
    Moderator,
    Admin,
    Owner
}

impl fmt::Display for Rank {
    fn fmt(
        &self, 
        f: &mut fmt::Formatter<'_>
    ) -> fmt::Result {
        write!(f, "{:?}", self)
    }
} 


#[derive(PartialEq, Debug)]
pub enum LoginMethod {
    Username,
    Email
}

impl fmt::Display for LoginMethod {
    fn fmt(
        &self, 
        f: &mut fmt::Formatter<'_>
    ) -> fmt::Result {
        write!(f, "{:?}", self)
    }
} 
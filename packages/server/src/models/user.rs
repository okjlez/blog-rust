use std::default;
use std::time::{self, UNIX_EPOCH};

use diesel::{expression::AsExpression, FromSqlRow};
use diesel::sql_types::*;
use rocket::serde::{Serialize, Deserialize};


#[derive(
    Serialize, 
    Deserialize, 
    Clone
)]
#[serde(crate = "rocket::serde")]
struct Account {
    #[ignore]
    id: String,
    username: Option<String>,
    password: Option<String>,
    email: Option<String>,
    #[ignore]
    rank: Rank
}

impl Default for Account {
    fn default() -> Self {
        let a = time::SystemTime::now();
        let b = a.duration_since(UNIX_EPOCH);
        let c = b.unwrap().as_nanos();
        Account { 
            id: c.to_string(),
            username: None, 
            password: None, 
            email: None, 
            rank: Rank::default() 
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq, 
    Serialize, 
    Deserialize, 
    AsExpression, 
    FromSqlRow,
    Default
)]
#[sql_type = "SmallInt"]
#[serde(crate = "rocket::serde")]
enum Rank {
    /// Will be represented as  0
    #[default]
    None,
    /// Will be represented as  1
    Member,
    /// Will be represented as  2
    Moderator,
    /// Will be represented as  3
    Admin,
    /// Will be represented as  4
    Owner
}

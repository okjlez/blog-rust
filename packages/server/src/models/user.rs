use std::time::{self, UNIX_EPOCH};
use postgres_types::{ToSql, FromSql};
use rocket::serde::{Serialize, Deserialize};

#[derive(
    Clone,
    Serialize, 
    Deserialize, 
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

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq, 
    Serialize, 
    Deserialize, 
    ToSql,
    FromSql
)]
#[serde(crate = "rocket::serde")]
enum Rank {
    #[default]
    None,
    Member,
    Moderator,
    Admin,
    Owner
}

impl Default for Account {
    fn default() -> Self {
        let id_a = time::SystemTime::now();
        let id_b = id_a.duration_since(UNIX_EPOCH);
        let id_c = id_b.unwrap().as_nanos();
        Account { 
            id: id_c.to_string(),
            username: None, 
            password: None, 
            email: None, 
            rank: Rank::default() 
        }
    }
}

impl Account {
    //create
    //new
}

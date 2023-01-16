use std::{sync::Arc, time::{self, UNIX_EPOCH}};

use deadpool_postgres::Pool;
use pbkdf2::{password_hash::SaltString};
use postgres_types::ToSql;
use rand_core::OsRng;
use rocket::serde::{Deserialize, Serialize};
use tokio_postgres::types::FromSql;
use std::borrow::Cow;
use pbkdf2::{
    password_hash::{
        PasswordHasher
    },
    Pbkdf2
};

use crate::error::Error;

pub struct AccountConfig {
    pg_pool: Arc<Pool>
}

impl AccountConfig {
    pub fn new(pg_pool: Pool) -> AccountConfig {
        Self {
            pg_pool: Arc::new(pg_pool)
        }
    }

    pub async fn create_account(&self, acc: Account) -> Result<(), Error>{
        let pg = &self.pg_pool.get()
        .await.unwrap();
        let salt = SaltString::
            new(&acc.password_salt.as_str()).unwrap();
        let password = Pbkdf2.hash_password(
            &acc.password.as_bytes(), 
            &salt).unwrap();
        let sql = "
        SELECT create_account(
            $1, $2, $3, $4, $5, $6
        )";
        let stmt = pg.prepare(&sql).await.unwrap();
        let query = pg.query(
            &stmt, 
            &[
                &acc.id,
                &acc.username,
                &acc.email,
                &password.hash
                    .unwrap()
                    .to_string(),
                &acc.password_salt,
                &acc.rank
                ]).await;
        match query {
            Ok(_) => {
                println!("{}", "Successfully created an account.");
                //log the account here...
                Ok(())
            },
            Err(er) => {
                let error_message = er
                .as_db_error().unwrap()
                .message();
                Err(Error::UniqueViolation(error_message.to_string()))
            },
        }
    }
}

#[derive(
    Debug,
    Clone,
    Serialize, 
    Deserialize
)]
#[serde(crate = "rocket::serde")]
pub struct Account {
    #[ignore]
    id: String,
    username: String,
    password: String,
    #[ignore]
    password_salt: String,
    email: String,
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
pub enum Rank {
    #[default]
    None,
    Member,
    Moderator,
    Admin,
    Owner
}

impl Account {
    pub fn new<'c, V>(
        username: V, 
        password: V, 
        email: V
    ) -> Self 
    where V: Into<Cow<'c, str>> {
        let mut acc = Account::default();
        acc.username = username.into().to_string();
        acc.password = password.into().to_string();
        acc.email = email.into().to_string();
        acc
    }

}
impl Default for Account { 
    fn default() -> Self {
        Account { 
            id: time::SystemTime::now()
            .duration_since(UNIX_EPOCH).unwrap()
            .as_nanos().to_string(),
            username: "".to_string(), 
            password: "".to_string(), 
            password_salt: SaltString::
            generate(&mut OsRng)
            .as_salt().to_string(),
            email: "".to_string(), 
            rank: Rank::default() 
        }
    }
}
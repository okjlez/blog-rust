use std::{sync::Arc, time::{self, UNIX_EPOCH, SystemTime}};

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

    pub fn create() -> Result<(), Error> {
        todo!()
    }
    
    // anything prefixed with quik is used to help split
    // the code into smaller chunks. the function name should
    // give you an idea of what it does.
    pub fn quik_pass(acc: &Account, password: String) -> String {
        let salt_s = acc.password_salt.as_str();
        let salt = SaltString::new(salt_s).unwrap();
        Pbkdf2.hash_password(
            password.as_bytes(), 
            &salt
        ).unwrap().to_string()
    }

    pub fn quik_salt() -> String {
        let salt = SaltString::generate(&mut OsRng);
        salt.as_salt().to_string()
    }

    pub fn quik_id() -> String {
        let st = SystemTime::now();
        let st_ds = 
            st.duration_since(UNIX_EPOCH).unwrap();
        st_ds.as_nanos().to_string()
    }
    /* 
    pub async fn create_account(&self, acc: Account) -> Result<(), Error>{
        let pg = &self.pg_pool.get()
        .await.unwrap();
        let salt = SaltString::
            new(&acc.password_salt.as_str()).unwrap();
        
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
                let error_message = er.as_db_error().unwrap()
                .message();
                let error_code = er.as_db_error()
                .unwrap()
                .code()
                .code().to_string();
                //constraint violation
                if error_code.eq("23514") {
                    return Err(
                        Error::ConstraintViolation(
                            error_message.to_string()
                        ))
                }
                //unique violation..
                Err(Error::UniqueViolation(
                    error_message.to_string()
                ))
            },
        }
    }
    */
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
        let pass = password.into().to_string();
        acc.username = username.into().to_string();
        acc.password = AccountConfig::quik_pass(&acc, pass);
        acc.email = email.into().to_string();
        acc
    }
}
impl Default for Account { 
    fn default() -> Self {
        Account { 
            id: AccountConfig::quik_id(),
            username: String::new(), 
            password: String::new(), 
            password_salt: AccountConfig::quik_salt(),
            email: String::new(), 
            rank: Rank::default()
        }
    }
}
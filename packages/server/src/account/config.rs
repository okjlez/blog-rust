
#![feature(allocator_api)]
use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}, borrow::Cow, result};
use std::fs::File;
use std::path::Path;

use deadpool_postgres::Pool;
use pbkdf2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString, Salt
    },
    Pbkdf2
};
use postgres_types::{ToSql, FromSql};
use rocket::{serde::{Serialize, Deserialize, self}, route, http::Cookie, FromForm, request::{FromRequest, self}, Request};
use tokio_postgres::{Row, Column, SimpleQueryMessage};


use crate::session::config::Session;

use super::{enums::{Rank, LoginMethod}, error::AccountError, routes};

/// Simple struct that helps select,insert,update and delete rows
/// from the postgres database (for the account table :0).
pub struct AccountConfig<'a> {
    // The deadpool_pg pool instance (wrapped in an arc.).
    pub pg_pool: &'a Pool
}

impl<'a> AccountConfig<'a> {
    /// Constructs a new [`AccountConfig`]. This method is used to help
    /// instantiate the deadpool-postgres pool. A necessity.
    /// for this class. 
    ///
    /// # Example
    ///
    /// Create an AccountConfig.
    ///
    /// ```rust
    /// use account::config::AccountConfig;
    ///
    /// let acc_config = AccountConfig::new("table_name", dpg_pool);
    /// ```
    #[inline(always)]
    pub fn new(pg_pool: &'a Pool) -> Self {
        let acc_config = AccountConfig {
            pg_pool: &pg_pool
        };
        acc_config
    }

    /// Creates a row inside the your table_name in Postgres
    ///
    /// # Example
    ///
    /// Creates a row inside the table_name table.
    ///
    /// ```rust
    /// use account::config::AccountConfig;
    ///
    /// let acc_config = AccountConfig::new("table_name", dpg_pool);
    /// let acc = Account::new("zeljko", "iloveyou", "zeljko@gmail.com");
    /// acc_config.create(acc);
    /// ```
    pub async fn create(&self, acc: Account) -> Result<(), AccountError>{
        let sql = "SELECT create_account($1, $2, $3, $4, $5)";
        let result = self.quik_query(sql, &[&acc.id(), acc.username(), acc.email(), acc.password(), acc.rank()]).await;
        match result {
            Ok(_) => Ok({
                println!("{}", "Success")
            }),
            Err(er) => {
                let db_error = er.as_db_error();
                let db_message = db_error.unwrap().message().to_string();
                Err(AccountError::parse_db_error(db_error, &acc, db_message))
            },
        }
    }

    /// Authenticate with your preferred method.
    ///
    /// # Example
    ///
    /// Creates a cookie after the authentication is successful.
    ///
    /// ```rust
    /// use account::config::AccountConfig;
    ///
    /// let acc_config = AccountConfig::new("table_name",  dpg_pool);
    /// 
    /// // login via username
    /// acc_config.auth(LoginMethod::Username, "zeljko", "password")
    /// 
    /// // login via email
    /// acc_config.auth(LoginMethod::Email, "ilovz@gmail.com", "password")
    /// ```
    pub async fn auth(&self, //refractor...
        method: LoginMethod, 
        key: &str, 
        pass: &str
    ) -> Result<Session, AccountError> {
        let sql = format!("SELECT * from accounts where {} ILIKE $1", method.to_string());
        let response = self.quik_query(&sql, &[&key]).await;
        match response {
            Ok(res) => Ok({
                if res.len() >= 1 {
                    let acc = Account::from(&res[0]);
                    let can_login = AccountConfig::quik_compare(&acc, &pass);
                    if can_login {
                        return Ok(Session::new(acc.id()))
                    } else {
                        return Err(AccountError::WrongPassword)
                    }
                }
                return Err(AccountError::AccountNotFound(key.to_string()))
            }),
            Err(_) => todo!(), // never gets called ? even when length is 0 hmm
        }
    }

    /// Finds an Account by their `field` inside the database and returns
    /// [`Account`].
    ///
    /// Available Fields: **id**, **username** and **email**.
    /// 
    /// # Example
    ///
    /// Find an account by their ID.
    ///
    /// ```rust
    /// use account::config::AccountConfig;
    ///
    /// let acc_config = AccountConfig::new(dpg_pool); 
    /// 
    /// let acc_by_id: Account = acc_config.find('id', '1673919920888240800');
    /// let acc_by_username: Account = acc_config.find('username', 'zeljko');
    /// let acc_by_email: Account = acc_config.find('email', "zeljko@gmail.com");
    /// let acc_by_session Account = acc_config.find('session', 'dDSwUKaRICtMOkQDRTB54');
    /// ``
    pub async fn find(&self, find: &str, value: &str) -> Result<Account, AccountError> {
        let sql = format!("select * from find_by_{}($1)", find);
        let response = self.quik_query(&sql, &[&value]).await;
        match response {
            Ok(res) => Ok({
                Account::from(&res[0])
            }),
            Err(er) => {
                Err(AccountError::AccountNotFound(value.to_string()))
            },
        }
    }

    //
    // Quik Functions
    //  (shorthands)
    // 

    /// A shorthand for generating a hashed password from the Pbkdf2 library.
    ///
    /// # Example
    /// 
    /// Create a hashed password.
    ///
    /// ```rust
    /// use account::config::AccountConfig;
    /// 
    /// let acc = Account::new("zeljko", "iloveyou", "zeljko@gmail.com");
    /// let salt = AccountConfig::quick_pass(&acc);
    /// println!("{}", salt); // UtCDtWw96w324K8NIW/YANc+aHvaCMvc9yeqiyDDDTw
    /// ```
    
    fn quik_hashpass(pass: &str) -> String {
        Pbkdf2.hash_password(
            pass.as_bytes(), &AccountConfig::quik_salt()).unwrap().to_string()
    }

    /// A shorthand for comparing two hashes from the Pbkdf2 library.
    ///
    /// # Example
    /// 
    /// Compares two hashed passwords.
    ///
    /// ```rust
    /// use account::config::AccountConfig;
    /// 
    /// let acc = Account::new("zeljko", "iloveyou", "zeljko@gmail.com");
    /// 
    /// // false
    /// let pass_comp_1 = AccountConfig::quik_compare(&acc, "idontloveyou");
    /// 
    /// // true
    /// let pass_comp_2 = AccountConfig::quik_compare(&acc, "iloveyou");
    /// ```
    fn quik_compare(acc: &Account, pass: &str) -> bool {
        let stored_pass = PasswordHash::new(acc.password()).unwrap();
        Pbkdf2.verify_password(pass.as_bytes(), &stored_pass).is_ok()
    }
    
    /// A shorthand for generating a salt from the pbkdf2 library.
    ///
    /// # Example
    /// 
    /// Create a salt.
    ///
    /// ```rust
    /// use account::config::AccountConfig;
    ///
    /// let salt = AccountConfig::quick_salt();
    /// println!("{}", salt); // AJJfAf2HCkUsVk4UaOg8uA
    /// ```
    #[inline(always)]
    fn quik_salt() -> SaltString {
        SaltString::generate(&mut OsRng)
    }

    /// A shorthand for generating a user's id.
    ///
    /// # Example
    ///
    /// Create an id.
    /// 
    /// ```rust
    /// use account::config::AccountConfig;
    ///
    /// let id = AccountConfig::quik_id();
    /// println!("{}", id); // 1673890867124778800
    /// ```
    #[inline(always)]
    fn quik_id() -> String {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        time.as_nanos().to_string()
    }

    /// A shorthand for executing queries this function
    /// should not be used outside this module.
    ///
    /// # Example
    ///
    /// Execute a postgres query
    /// 
    /// ```rust
    /// use account::config::AccountConfig;
    ///
    /// &self.quik_query("SELECT * from table_name");
    /// ```
    pub async fn quik_query(&self, sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, tokio_postgres::Error>
    {
        let pg = &self.pg_pool.get().await.unwrap();
        let stmt = pg.prepare(&sql).await.unwrap();
        pg.query(&stmt, params).await
    }

    pub async fn quik_simple_query(&self, sql: &str) ->
    Result<Vec<Row>, tokio_postgres::Error>
    {
        let pg = &self.pg_pool.get().await.unwrap();
        pg.query(sql, &[]).await
    }
}

/// The blueprint for an account. 
/// 
/// You can add and delete to your liking but make sure your table
/// reflects your changes.
/// 
/// Look at sql/create_table_accounts.sql for more information on
/// how to create a table.
/// 
/// All the 'meaty' logic should be handled in AccountConfig not here.
#[derive(Debug, Clone, ToSql, FromSql, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Account {
    #[serde(default)]
    id: String,
    username: String,
    password: String,
    email: String,
    #[serde(default)]
    rank: Rank
}

// you can easily add username support.. due to the AccountConfig#auth() method.
#[derive(FromForm)]
pub struct AccountLogin {
    pub email: String,
    pub password: String,
}

impl Account {
    /// Constructs a new [`Account`]. This method provides
    /// the data for the [`AccountConfig`] in order for it
    /// execute a variety of functions.
    ///
    /// # Example
    ///
    /// Create an Account.
    ///
    /// ```rust
    /// use account::config::AccountConfig;
    ///
    /// let acc = Account::new("zeljko", "iloveyou", "zeljko@gmail.com");
    /// 
    /// println!("{}", acc.username()) // zeljko
    /// ```
    #[inline(always)]
    pub fn new(username: &str, password: &str, email: &str) -> Self {
        let mut acc = Account::default();
        acc.username = username.to_string();
        acc.password = AccountConfig::quik_hashpass(password);
        acc.email = email.to_string();
        acc
    }

    // Returns the id of Account
    pub fn id(&self) -> &String {
        &self.id
    }

    // Returns the username of Account
    pub fn username(&self) -> &String {
        &self.username
    }

    // Returns the password of Account
    pub fn password(&self) -> &String {
        &self.password
    }

    // Returns the email of Account
    pub fn email(&self) -> &String {
        &self.email
    }

    // Returns the rank of Account
    pub fn rank(&self) -> &Rank {
        &self.rank
    }
}

/// The default configuration for the Account struct.
/// You may change it to your liking.
/// 
/// If you plan on changing the 'Rank' enum make sure
/// you update your Postgres 'Types' in the the data-
/// base.
/// 
/// Look at sql/create_types.sql how to create new
/// types in postgres.
/// 
/// Anything with an ! should be edited with caution.
impl Default for Account {
    fn default() -> Self {
        Self { 
            id: AccountConfig::quik_id(), 
            username: Default::default(), 
            password: Default::default(), // !
            email: Default::default(), 
            rank: Rank::default()  // !
        }
    }
}

impl From<&Row> for Account {
    fn from(value: &Row) -> Self {
        Account { 
            id: value.get(0), 
            username: value.get(1),
            email: value.get(2),  
            password: value.get(3), 
            rank: value.get(4)
        }
    }
}

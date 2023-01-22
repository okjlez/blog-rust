use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}, borrow::Cow};
use std::fs::File;
use std::path::Path;

use deadpool_postgres::Pool;
use pbkdf2::password_hash::{SaltString, PasswordHasher};
use pbkdf2::Pbkdf2;
use postgres_types::{ToSql, FromSql};
use rand_core::OsRng;
use rocket::{serde::{Serialize, Deserialize}, route};
use tokio_postgres::{Row, Column};

use super::{enums::Rank, error::AccountError, routes};

macro_rules! add_commit_prereq {
    ($conn:item) => {
        
    };
} 

/// Simple struct that helps select,insert,update and delete rows
/// from the postgres database (for the account table :0).
pub struct AccountConfig<'a> {
    // The name of the db table.
    table_name: String,
    // The deadpool_pg pool instance (wrapped in an arc.).
    pg_pool: &'a Pool
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
    pub fn new(table_name: &str, pg_pool: &'a Pool) -> Self {
        let acc_config = AccountConfig {
            table_name: table_name.to_string(),
            pg_pool: &pg_pool
        };
        acc_config
    }

    /// Executes all the prerequisite queries.
    ///
    /// # Example
    ///
    /// Executes a variety of set queries.
    ///
    /// ```rust
    /// use account::config::AccountConfig;
    ///
    /// let acc_config = AccountConfig::new("table_name", dpg_pool);
    /// acc_config.()
    /// acc_config.open_sesame();
    /// ```
    async fn open_sesame(&self) {
        let pg = &self.pg_pool.get().await.unwrap();
        pg.simple_query("query").await.unwrap();
        //add_commit_prereq!(pg)
        //load files find a way to do it i found
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
        let sql = "SELECT create_account($1, $2, $3, $4, $5, $6)";
        let result = &self.quik_query(sql, &[&acc.id(), acc.username(), acc.email(), acc.password(), acc.password_salt(), acc.rank()]).await;
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

    /* 
    fn update<'c, V>(&self, session_id: &str, update_field: &str, value: V) where V: Into<Cow<'c, str>> + Sync + ToSql {
        //session_manager::get_session("SESSION_ID THAT DIRECTLY LINNKED TO ACCOUNT") // DO QUERIES DA DA DA GET DATA AND STUFF YES. :)
    }
    */

    /// Finds an Account by their id inside the database and returns
    /// [`Account`]. Just for your convenience the query also returns
    /// created_at. 
    ///
    /// # Example
    ///
    /// Find an account by their ID.
    ///
    /// ```rust
    /// use account::config::AccountConfig;
    ///
    /// let acc_config = AccountConfig::new("table_name", dpg_pool); 
    /// let acc: Account = acc_config.find('1673919920888240800');
    /// ``
    pub async fn find(&self, account_id: &str) -> Result<Account, AccountError> {
        let sql = "select * from find_by_id($1)";
        let result = self.quik_query(sql, &[&account_id]).await;
        match result {
            Ok(res) => Ok({
                Account::from(&res[0])
            }),
            Err(er) => {
                Err(AccountError::AccountNotFound(account_id.to_string()))
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
    #[inline(always)]
    fn quik_hashpass(acc: &Account) -> String {
        let salt = SaltString::new(acc.password_salt()).unwrap();
        Pbkdf2.hash_password(
            acc.password().as_bytes(), &salt)
            .unwrap().hash.unwrap().to_string()
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
    fn quik_salt() -> String {
        SaltString::generate(&mut OsRng).as_salt().to_string()    
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
    async fn quik_query(&self, sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, tokio_postgres::Error> {
        let pg = &self.pg_pool.get().await.unwrap();
        let stmt = pg.prepare(&sql).await.unwrap();
        pg.query(&stmt, params).await
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
#[derive(Debug, ToSql, FromSql, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Account {
    #[serde(default)]
    id: String,
    username: String,
    password: String,
    #[serde(default)]
    password_salt: String,
    email: String,
    #[serde(default)]
    rank: Rank
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
        acc.password = AccountConfig::quik_hashpass(&acc);
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

    // Returns the password_salt of Account
    pub fn password_salt(&self) -> &String {
        &self.password_salt
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
            password_salt: AccountConfig::quik_salt(), // !
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
            password_salt: value.get(4), 
            rank: value.get(5)
        }
    }
}

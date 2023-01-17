use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}};

use deadpool_postgres::Pool;
use pbkdf2::password_hash::{SaltString, PasswordHasher};
use pbkdf2::Pbkdf2;
use rand_core::OsRng;

use super::enums::Rank;

/// Simple struct that helps select,insert,update and delete rows
/// from the postgres database (for the account table :0).
pub struct AccountConfig {
    // The name of the db table.
    table_name: String,
    // The deadpool_pg pool instance (wrapped in an arc.).
    pg_pool: Arc<Pool>
}

impl AccountConfig {
    /// Constructs a new [`AccountConfig`]. This method is used to help
    /// instantiate the the deadpool-postgres pool type. A necessity.
    /// for this class. 
    ///
    /// # Example
    ///
    /// Create an AccountConfig.
    ///
    /// ```rust
    /// use account::config::AccountConfig;
    ///
    /// let acc_config = AccountConfig::new("accounts", dpg_pool);
    /// ```
    #[inline(always)]
    pub fn new(table_name: &str, pg_pool: Pool) -> AccountConfig {
        Self {
            table_name: table_name.to_string(),
            pg_pool: Arc::new(pg_pool)
        }
    }
    //
    // Quik Functions
    //  (shorthands)
    // 

    /// A shorthand for generating a hashed password from the pbkdf2 library.
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
    pub fn quik_pass(acc: &Account) -> String {
        let salt = SaltString::new(acc.password_salt()).unwrap();
        Pbkdf2.hash_password(
            acc.password().as_bytes(), &salt).unwrap()
            .hash.unwrap().to_string()
    }
    
    /// A shorthand for generating a salt from the pbkdf2 library.
    ///
    /// # Example
    /// 
    /// Create a salt that is used to hash a password.
    ///
    /// ```rust
    /// use account::config::AccountConfig;
    ///
    /// let salt = AccountConfig::quick_salt();
    /// println!("{}", salt); // AJJfAf2HCkUsVk4UaOg8uA
    /// ```
    #[inline(always)]
    pub fn quik_salt() -> String {
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
    pub fn quik_id() -> String {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        time.as_nanos().to_string()
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
    pub fn new(username: &str, password: &str, email: &str) -> Self {
        let mut acc = Account::default();
        acc.username = username.to_string();
        acc.password = AccountConfig::quik_pass(&acc);
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

    // Returns the username of Account
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
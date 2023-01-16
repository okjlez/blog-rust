use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}};

use deadpool_postgres::Pool;
use pbkdf2::password_hash::SaltString;
use rand_core::OsRng;

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
    /// println!("{}", salt); // VYobWWE+SQet2LQl9t5E9Q
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
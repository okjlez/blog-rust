/// A simple, yet effective account system.

use std::{time::{self, UNIX_EPOCH}, borrow::Cow};
use async_trait::async_trait;
use deadpool_postgres::Object;
use postgres_types::{ToSql, FromSql};
use rocket::serde::{Serialize, Deserialize};

use crate::traits::query::QueryCrud;

/// Represents the config of an individual
/// allows flexibility among the codebase.
pub struct AccountConfig<'a> {
    query: &'a Object,
    acc: Account
}

/// Represents an individual's account.
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
    username: Option<String>,
    password: Option<String>,
    email: Option<String>,
    #[ignore]
    rank: Rank
}

/// The enum Rank shows all the
/// available ranks.
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

/// The enum FIELD shows all the fields
/// Account type has.
pub enum AccountField {
    Id,
    Username,
    Password,
    Email,
    Rank
}

impl Default for Account {
    fn default() -> Self {
        let id_a = time::SystemTime::now();
        let id_b = id_a.duration_since(UNIX_EPOCH);
        let id = id_b.unwrap().as_nanos().to_string();
        Account { 
            id,
            username: None, 
            password: None, 
            email: None, 
            rank: Rank::default() 
        }
    }
}

impl Account {
    /// creates a new account with the given username, password and email.
    ///
    /// # Arguments
    ///
    /// * `username` - the name of the account.
    /// * `password` - the password of the account.
    /// * `email` - the email of the account.
    ///
    /// # Examples
    /// 
    /// ```
    ///
    /// // Import our ['Account'] struct.
    /// use models::user::Account;
    /// use models::user::AccountBuilder;
    /// // instantiate the account object.
    /// let account = Account::new("CrazyJohn", "qwerty", "john@gmail.com");
    /// // Prints out the id of the account.
    /// println!("{}", account.id)
    /// // if you prefer the builder design, you can do this.
    /// let account_builder = Account::builder()
    ///    .set_username("email")
    ///    .set_password("asdasad")
    ///    .set_email("hello@gmail.com")
    ///    .build();
    /// println!("{}", account_builder.id)
    /// ```
    pub fn new<'c, V>(
        username: V, 
        password: V, 
        email: V
    ) -> Self 
    where V: Into<Cow<'c, str>> {
        let mut acc = Account::default();
        acc.username = Some(username.into().to_string());
        acc.password = Some(password.into().to_string());
        acc.email = Some(email.into().to_string());
        acc
    }
    
    /// An modifier function that grants you the ability
    /// to create/read/update/delete account values of an indivi-
    /// ual account inside the database. Hence the name m-
    /// (modifier).
    pub fn m(self, pg_obj: &Object) -> AccountConfig {
        return AccountConfig { query: pg_obj, acc: self }
    }

    /// If you would rather use the builder pattern.
    pub fn builder() -> AccountBuilder {
        AccountBuilder::default()
    }

    /// Returns the id of the account.
    pub fn id(self) -> String {
        self.id
    }

    /// Returns the username of the account.
    pub fn username(self) -> String {
        self.username.unwrap()
    }
    
    /// Returns the password of the account.
    pub fn password(self) -> String {
        self.password.unwrap()
    }

    /// Returns the email of the account.
    pub fn email(self) -> String {
        self.email.unwrap()
    }
    
    /// Returns the rank of the account.
    pub fn rank(self) -> Rank {
        self.rank
    }
}

impl AccountConfig<'_> {
    /// Check if the indicated value already exists.
    ///
    /// # Arguments
    ///
    /// * `check_for` - the field to check for.
    /// * `value` - the value to check for.
    ///
    /// # Examples
    /// 
    /// ```
    ///
    /// START HERE
    /// ```
    pub fn exists<'c, V>(
        &self,
        check_for: AccountField,
        value: V
    ) -> Result<(), crate::error::Error>
    where V: Into<Cow<'c, str>> + Into<Rank> {
        todo!()
    }
}

#[async_trait]
impl QueryCrud for AccountConfig<'_> {
    
    async fn create(&self) -> Result<(), crate::error::Error> {
        todo!()
    }

    async fn read(&self) -> Result<(), crate::error::Error> {
        todo!()
    }

    async fn update(&self) -> Result<(), crate::error::Error> {
        todo!()
    }

    async fn delete(&self) -> Result<(), crate::error::Error> {
        todo!()
    }
}

#[derive(Default)]
pub struct AccountBuilder {
    account: Account
}

impl AccountBuilder {
    
    pub fn set_username(mut self, username: &str) -> Self {
        self.account.username = Some(username.to_string());
        self
    }
    
    pub fn set_password(mut self, password: &str) -> Self {
        self.account.password = Some(password.to_string());
        self
    }

    pub fn set_email(mut self, email: &str) -> Self {
        self.account.email = Some(email.to_string());
        self
    }

    pub fn build(self) -> Account {
        self.account
    }
}

use std::{time::{self, UNIX_EPOCH}, borrow::Cow};
use postgres_types::{ToSql, FromSql};
use rocket::serde::{Serialize, Deserialize};

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
    pub username: Option<String>,
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
pub enum Rank {
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
    /// // instantiate the account object.
    /// let account = Account::new("CrazyJohn", "qwerty", "john@gmail.com");
    /// // Prints out the id of the account.
    /// println!("{}", account.id)
    /// 
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

    pub fn builder() -> AccountBuilder {
        AccountBuilder::default()
    }

    pub fn id(self) -> String {
        self.id
    }

    pub fn username(self) -> String {
        self.username.unwrap()
    }
    
    pub fn password(self) -> String {
        self.password.unwrap()
    }

    pub fn email(self) -> String {
        self.email.unwrap()
    }
    
    pub fn rank(self) -> Rank {
        self.rank
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

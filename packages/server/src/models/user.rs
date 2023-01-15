use core::fmt;
/// A simple, yet effective account system.

use std::{time::{self, UNIX_EPOCH}, borrow::{Cow}};
use async_trait::async_trait;
use deadpool_postgres::Object;
use nanoid::format;
use postgres_types::{ToSql, FromSql};
use rocket::serde::{Serialize, Deserialize};

use crate::{traits::query::QueryCrud, error::Error};

pub struct AccountConfig<'a> {
    query: &'a Object,
    acc: Account
}

#[derive(Debug, PartialEq)]
pub enum AccountField {
    Id,
    Username,
    Password,
    Email,
    Rank
}

impl AccountField {
    fn obtain(
        &self, 
        acc: Account
    ) -> Option<String> {
        match self {
            AccountField::Id => return Some(acc.id),
            AccountField::Username => return acc.username,
            AccountField::Password => return acc.password,
            AccountField::Email => return acc.email,
            AccountField::Rank => return None,
        }
    }
}

impl fmt::Display for AccountField {
    fn fmt(
        &self, 
        f: &mut fmt::Formatter<'_>
    ) -> fmt::Result {
        write!(f, "{:?}", self)
    }
} 

impl AccountConfig<'_> {
    pub async fn exists(
        &self,
        check_for: AccountField //cf
    ) -> bool {
        let query = self.query;
        let cfe = check_for.to_string();
        let cfv = AccountField::obtain(
            &check_for, 
            self.acc.to_owned()).unwrap();
        let a = format!("
            SELECT {} FROM accounts WHERE {} = $1", cfe, cfe 
        );
        let b = query.
            prepare(&a).await.unwrap();
        let c = query.
            query(&b, &[&cfv]).await.unwrap();
        return !c.is_empty();
    }

}

#[async_trait]
impl QueryCrud for AccountConfig<'_, > {
    async fn create(&self) -> Result<(), Error> {

        /*
        let acc = &self.acc;
        let qry = self.query;
        let a = "
        INSERT INTO accounts (
            id,
            username,
            email,
            password,
            rank
        ) VAlUES (
            $1,
            $2,
            $3,
            $4,
            $5
        )";
        let b = qry
            .prepare(&a)
            .await.unwrap();
        qry
            .query(&b, 
                &[&acc.id, &acc.username.clone().unwrap(), 
                &acc.email.clone().unwrap(), 
                &acc.password.clone().unwrap(), 
                &acc.rank])
            .await.unwrap();
        Ok(())
        */
        todo!()
    }

    async fn read(&self) -> Result<(), Error> {
        todo!()
    }

    async fn update(&self) -> Result<(), Error> {
        todo!()
    }

    async fn delete(&self) -> Result<(), Error> {
        todo!()
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
    pub fn m(
        self, 
        pg_obj: &Object
    ) -> AccountConfig {
        return AccountConfig { query: pg_obj, acc: self }
    }

    /// If you would rather use the builder pattern.
    pub fn builder() -> AccountBuilder {
        AccountBuilder::default()
    }

    /// Returns the id of the account.
    pub fn id(
        self
    ) -> String {
        self.id
    }

    /// Returns the username of the account.
    pub fn username(
        self
    ) -> String {
        self.username.unwrap()
    }
    
    /// Returns the password of the account.
    pub fn password(
        self
    ) -> String {
        self.password.unwrap()
    }

    /// Returns the email of the account.
    pub fn email(
        self
    ) -> String {
        self.email.unwrap()
    }
    
    /// Returns the rank of the account.
    pub fn rank(
        self
    ) -> Rank {
        self.rank
    }
}

#[derive(Default)]
pub struct AccountBuilder {
    account: Account
}

impl AccountBuilder {
    pub fn set_username(
        mut self, 
        username: &str
    ) -> Self {
        self.account.username = Some(username.to_string());
        self
    }
    
    pub fn set_password(
        mut self, 
        password: &str
    ) -> Self {
        self.account.password = Some(password.to_string());
        self
    }

    pub fn set_email(
        mut self, 
        email: &str
    ) -> Self {
        self.account.email = Some(email.to_string());
        self
    }

    pub fn build(
        self
    ) -> Account {
        self.account
    }
}


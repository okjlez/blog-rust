use std::{sync::{Arc, RwLock}, time::{self, UNIX_EPOCH}, borrow::Cow};

use deadpool_postgres::{Object, Pool};
use postgres_types::{ToSql, FromSql};
use rocket::serde::{Deserialize, Serialize};

use crate::error::Error;

// Contains communication
pub struct AccountConfig {
    pg_pool: Arc<Pool>
}

impl AccountConfig {
    pub fn new(pg_pool: Pool) -> AccountConfig {
        Self {
            pg_pool: Arc::new(pg_pool)
        }
    }
    
    pub async fn account_exists<'c, V>(
        &self, 
        field: &str, 
        search_for: V
    ) -> bool
    where V: Into<Cow<'c, str>> + Sync + ToSql {
        let object = &self.pg_pool.get().await.unwrap();
        let sql = format!("
        SELECT EXISTS 
        (
            SELECT 1 from accounts 
            WHERE {} = $1
        );", field.to_string());
        let a = object.prepare(&sql).await.unwrap();
        let b = object.query(&a, &[&search_for]).await.unwrap();
        let result: bool = b[0].get(0);
        result
    }

    /* 
    pub fn create(&self, acc: Account){}
    pub fn update(&self, acc: Account){}
    pub fn delete(&self, acc: Account){}
    pub fn load(&self, acc: Account){}*/
}

pub struct Account {
    #[ignore]
    id: String,
    username: Option<String>,
    password: Option<String>,
    email: Option<String>,
    #[ignore]
    rank: AccountRank
}

#[derive(
    Default,
    Debug,
    PartialEq, 
    Serialize, 
    Deserialize, 
    ToSql,
    FromSql
)]
#[serde(crate = "rocket::serde")]
pub enum AccountRank {
    #[default]
    None,
    Member,
    Moderator,
    Admin,
    Owner
}

impl Account {
    pub fn new(
        name: &str, 
        pass: &str, 
        email: &str
    ) -> Account {
        let mut acc = Account::default();
        acc.username =  Some(name.to_string());
        acc.password = Some(pass.to_string());
        acc.email = Some(name.to_string());
        acc
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn username(&self) -> &String {
        &self.username.as_ref().unwrap()
    }

    pub fn password(&self) -> &String {
        &self.password.as_ref().unwrap()
    }

    pub fn email(&self) -> &String {
        &self.email.as_ref().unwrap()
    }
    
    pub fn rank(&self) -> &AccountRank {
        &self.rank
    }
}

impl Default for Account {
    fn default() -> Self {
        Self { 
            id: time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos().to_string(), 
            username: Default::default(), 
            password: Default::default(), 
            email: Default::default(),
            rank: AccountRank::default(),
        }
    }
}
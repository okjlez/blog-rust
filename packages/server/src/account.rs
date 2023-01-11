/*!
   The Account module provides a way for users to authenticate themselves. It stores the essential information needed for any authentication system, including an ID, username, password, email, and role. 
   
   The 'role' indicates the group or access level of the user. 
   
   This module can be used as a foundation for building an authentication system in your application.
*/

use core::fmt;
use std::any::TypeId;

use deadpool_postgres::Object;
use rocket::{serde::{Deserialize, Serialize}, data::{FromData, Outcome}};
use tokio_postgres::types::ToSql;

use crate::error::Error;
use crate::error::Error::AccountNotFound;
use crate::error::Error::UsernameTaken;
use crate::error::Error::EmailTaken;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Account {
    id: String,
    username: String,
    password: String,
    email: String,
    role: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
  struct AccountJsonPost {
    username: String,
    password: String,
    email: String,
}

impl Account {    
    pub fn new(
        id: String,
        username: String,
        password: String,
        email: String,
        role: String,
    ) -> Self {
        Self {
            id,
            username,
            password,
            email,
            role,
        }
    }
}

pub struct AccountManager<'a> {
    pg: &'a Object,
    account: Account,
}

#[derive(Debug, PartialEq)]
pub enum AccountField {
    Id,
    Username,
    Password,
    Email,
    Role,
}

impl fmt::Display for AccountField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// AccountManager manages 
impl AccountManager<'_> {
    /// Constructs a new `AccountManager` from the given [Object, 'Account']
    pub fn new(pg: &Object, account: Account) -> AccountManager {
        AccountManager { pg, account }
    }

    pub async fn create(&self) -> Result<(), crate::error::Error> {
        let pg_obj = self.pg;

        let acc_name = &self.account.username;
        let acc_email = &self.account.email;
        let name_condition = Self::account_exists(
            pg_obj,
            AccountField::Username,
            acc_name   
        );
        let email_condition = Self::account_exists(
            pg_obj,
            AccountField::Email,
            acc_email
        );
        //find a better way of doing this...
        if name_condition.await { 
            return Err(UsernameTaken(acc_name.to_string())) }
        if email_condition.await {
            return Err(EmailTaken(acc_email.to_string()))
        }
        let pg_obj = &self.pg;
        let acc_id = &self.account.id;
        let stmt_q = "
        INSERT INTO accounts (
            id,
            username,
            email,
            password,
            role
        ) VAlUES (
            $1,
            $2,
            $3,
            $4,
            $5
        )";
        let stmt = pg_obj.prepare(stmt_q).
        await.unwrap();
        pg_obj.query(&stmt, 
            &[
                &acc_id,
                &acc_name,
                &acc_email,
                &self.account.password,
                &self.account.role
            ]).await.unwrap();
        return Ok(());
    }
    
    pub async fn update<V>(
        &self, 
        field: AccountField, 
        value: V
    ) -> Result<(), Error>
    where
        V: std::fmt::Display + 
        std::fmt::Debug + 
        'static + 
        ToSql + 
        Sync,
    {
        let pg_obj = &self.pg;
        let acc_id = &self.account.id;
        let condition = Self::account_exists(
            pg_obj, 
            AccountField::Id, 
            acc_id
        ).await;
        match condition {
            true => {
                let stmt_q = format!("
                    UPDATE 
                    accounts SET {} = $1
                    WHERE id = $2", 
                    field.to_string()
                );
                let stmt = pg_obj.prepare(&stmt_q).
                await.unwrap();
                pg_obj.query(&stmt, &[&value, acc_id])
                .await.unwrap();
                return Ok(())
            },
            false => {
                return Err(AccountNotFound(acc_id.to_string()));
            },
            _=> { unreachable!("How did you even get here?") }
        }
    }

    async fn account_exists<V>(
        pg_obj: &Object, 
        find_by: AccountField, 
        value: &V
    ) -> bool
    where
        V: std::fmt::Display + 
        std::fmt::Debug + 
        'static + 
        ToSql + 
        Sync,
    {
        if find_by == AccountField::Role && 
        TypeId::of::<V>() != TypeId::of::<i32>() { 
            println!("The field 'role' cannot be a string.");
            return false;
        }
        let stmt_q = format!("
            SELECT * FROM accounts 
            WHERE {}=$1", 
            find_by.to_string()
        );
        let stmt = pg_obj.prepare(&stmt_q)
        .await.unwrap();
        return !pg_obj.query(&stmt, &[&value])
        .await.unwrap().is_empty();
    }
}


pub mod posts {
    use std::time;

    use deadpool_postgres::{Object};
    use rocket::{Route, routes, post, serde::{json::Json, Deserialize, Serialize}, State};
    use serde_json::{Value, json};
    use std::time::UNIX_EPOCH;


    use super::{AccountManager, Account, AccountJsonPost};
    
    #[post("/account/new", format = "application/json", data = "<account>")]
    async fn account_new(account: Json<AccountJsonPost>, object: &State<Object>) 
    -> Value {
        let data = account; 
        let man = 
        AccountManager::new(
            object.inner(), 
            Account::new(
                time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos().to_string(), 
                data.0.username, 
                data.0.password, 
                data.0.email,
                "member".to_string()
            )
        );
        match man.create().await {
            Ok(_) => {
                return json!({"status" : "SUCCESS"})
            },
            Err(v) => {
                return json!({"status" : "FAILED", "reason": v.to_string()})
            }
        }
        
    }
    pub fn routes() -> Vec<Route> {
        routes![account_new]
    }
}

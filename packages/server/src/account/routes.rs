use deadpool_postgres::{Pool, Manager};
use rocket::form::Form;
use rocket::http::{CookieJar, Cookie};
use rocket::{serde::json::Json, post, Route};
use rocket::{routes, State};
use serde_json::{Value, json};
use crate::session::config::Session;

use super::config::{Account, AccountConfig, AccountLogin};
use super::enums::LoginMethod;

//simplify
#[post("/account/new", format = "application/json", data = "<_acc>")]
pub async fn account_new(_acc: Json<Account>, pool: &State<Pool>) -> Value {
    let account = Account::new(_acc.username(), _acc.password(), _acc.email());
    let acc_cfg = AccountConfig::new("accounts", pool.inner());
    match acc_cfg.create(account).await {
        Ok(_) => {
            return json!({"status" : "Success"})
        },
        Err(v) => {
            return json!({"status" : "FAILED", "reason": v.to_string()})
        }
    }
}


#[post("/account/login", data = "<login>")]
pub async fn account_login(jar: &CookieJar<'_>,login: Form<AccountLogin>, pool: &State<Pool>) -> Value {
    let acc_cfg = AccountConfig::new("accounts", pool.inner());

    match acc_cfg.auth(LoginMethod::Email, &login.email, login.password.to_string()).await {
        Ok(val) => {
            jar.add(Cookie::new("sid", val.session_id));
            return json!({"status" : "Success session_added"})
        },
        Err(v) => {
            return json!({"status" : "FAILED", "reason": v.to_string()})
        }
    }
}



pub fn routes() -> Vec<Route> {
    routes![account_new, account_login]
}
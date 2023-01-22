use deadpool_postgres::{Pool, Manager};
use rocket::{serde::json::Json, post, Route};
use rocket::{routes, State};

use serde_json::{Value, json};

use super::config::{Account, AccountConfig};


#[post("/account/new", format = "application/json", data = "<_acc>")]
pub async fn account_new(_acc: Json<Account>, pool: &State<Pool>) -> Value {
    let account = Account::new(_acc.username(), _acc.password(), _acc.email());
    let acc_cfg = AccountConfig::new("accounts", pool.inner());
    match acc_cfg.create(account).await {
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
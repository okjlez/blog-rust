use std::fmt::Debug;

use deadpool_postgres::{Pool, Manager};
use reqwest::StatusCode;
use rocket::form::Form;
use rocket::http::{CookieJar, Cookie, SameSite};
use rocket::request::{FromRequest, self};
use rocket::response::Redirect;
use rocket::{serde::json::Json, post, Route};
use rocket::{routes, State, Request, get};
use serde_json::{Value, json};
use crate::session::config::Session;
use crate::thread;
use crate::thread::config::{self, Thread};
use rocket::http::Status;

use super::config::{Account, AccountConfig, AccountLogin};
use super::enums::LoginMethod;
use super::error;


#[post("/account/new", format = "application/json", data = "<_acc>")]
pub async fn account_new(_acc: Json<Account>, pool: &State<Pool>) -> Value {
    let account = Account::new(_acc.username(), _acc.password(), _acc.email());
    let acc_cfg = AccountConfig::new(pool.inner());
    match acc_cfg.create(account).await {
        Ok(_) => {
            return json!({"status" : "SUCCESS"})
        },
        Err(v) => {
            return json!({"status" : "FAILED", "reason": v.to_string()})
        }
    }
}

#[post("/account/login", data = "<login>")]
pub async fn account_login(jar: &CookieJar<'_>, login: Form<AccountLogin>, pool: &State<Pool>) -> Result<Redirect, Status> {
    let cfg = AccountConfig::new(pool.inner());
    let email = &login.email;
    let password = &login.password;
    let is_auth = cfg.auth(LoginMethod::Email, &email, password).await;
    match is_auth {
        Ok(res) => {
            res.save(cfg).await;
            let sid = Cookie::new("sid", res.session_id);
            jar.add(sid);
            return Ok(
                Redirect::to("http://127.0.0.1:5173")
            );
        },
        Err(_) => {
            return Err(Status::NotFound);
        },
    }
}

#[get("/account/logout")]
pub async fn account_logout(jar: &CookieJar<'_>) -> Redirect {
    jar.remove(Cookie::named("sid"));
    Redirect::to("http://127.0.0.1:5173")
}

pub fn routes() -> Vec<Route> {
    routes![account_new, account_login, account_logout]
}
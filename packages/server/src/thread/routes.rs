use deadpool_postgres::Pool;
use rocket::{http::CookieJar, serde::json::Json, State, Route, routes, post, form::Form, response::Redirect, get};

use crate::account::config::AccountConfig;

use super::config::{Thread, ThreadManager};

#[post("/thread/new", data = "<_thread>")]
pub async fn thread_new(jar: &CookieJar<'_>, _thread: Form<Thread>, pool: &State<Pool>) {
    let cfg = AccountConfig::new(pool);
    let find_acc = cfg.find("session", jar.get("sid").unwrap().value()).await.unwrap().clone();
    let thread = Thread::new(&_thread.title(), &_thread.body(), &find_acc.id());

    thread.save(cfg).await; 
}

pub fn routes() -> Vec<Route> {
    routes![thread_new]
}
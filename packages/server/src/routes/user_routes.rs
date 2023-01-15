use rocket::Route;
use rocket::post;
use rocket::routes;
use serde_json::Value;

#[post("/account/new", format = "application/json")]
async fn account_new() -> Value {
    todo!()
}

pub fn routes() -> Vec<Route> {
    routes![account_new]
}
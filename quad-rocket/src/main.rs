#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate diesel;
extern crate quad_bank;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate serde;
extern crate serde_json;

mod db;
mod errors;

use errors::Error;
use quad_bank::models::Account;
use rocket_contrib::{JSON, Value};

#[derive(Deserialize)]
struct CreateAccountRequest {
    username: String,
    balance: i32,
}

#[derive(Deserialize)]
struct TransferRequest {
    dst_username: String,
    amount: i32,
}

#[get("/")]
fn show_accounts(conn: db::Conn) -> Result<JSON<Vec<Account>>, Error> {
    let accounts = quad_bank::all_accounts(&conn)?;
    Ok(JSON(accounts))
}

#[get("/<username>")]
fn show_account(username: String, conn: db::Conn) -> Result<JSON<Account>, Error> {
    let account = quad_bank::account_for_username(&conn, &username)?;
    Ok(JSON(account))
}

#[post("/", format="application/json", data="<request>")]
fn create_account(request: JSON<CreateAccountRequest>,
                  conn: db::Conn) -> Result<JSON<Account>, Error> {
    let account = quad_bank::create_account(
        &conn,
        &request.username,
        request.balance)?;

    Ok(JSON(account))
}

#[post("/<username>/transfer", format="application/json", data="<request>")]
fn transfer(username: String,
            request: JSON<TransferRequest>,
            conn: db::Conn) -> Result<JSON<Value>, Error> {
    quad_bank::transfer(
        &conn,
        &username,
        &request.dst_username,
        request.amount)?;

    Ok(JSON(json!({
        "msg": "transfer completed",
    })))
}

fn main() {
    let pool = db::init_pool("../db.sqlite");

    rocket::ignite()
        .manage(pool)
        .mount("/", routes![show_accounts, show_account, create_account, transfer])
        .launch();
}

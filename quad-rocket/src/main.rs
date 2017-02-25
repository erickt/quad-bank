#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate diesel;
extern crate quad_diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate serde;

mod db;
mod errors;

use diesel::Connection;
use errors::Error;
use quad_diesel::models::{Account, NewAccount};
use rocket_contrib::{JSON, Value};

/// The body request for creating an account.
#[derive(Deserialize)]
struct CreateAccountRequest {
    /// The name of the account to create.
    username: String,

    /// How many quadbucks to put in this account.
    balance: i32,
}

/// The http body request for transfering quadbucks to an account.
#[derive(Deserialize)]
struct TransferRequest {
    /// The recipient account of the quadbucks.
    username: String,

    /// How many quadbucks should we give to this account?
    amount: i32,
}

#[get("/")]
fn show_accounts(conn: db::Conn) -> Result<JSON<Vec<Account>>, Error> {
    let accounts = Account::all(&conn)?;
    Ok(JSON(accounts))
}

#[get("/<username>")]
fn show_account(username: String, conn: db::Conn) -> Result<JSON<Account>, Error> {
    let account = Account::find_by_username(&conn, &username)?;
    Ok(JSON(account))
}

#[post("/", format="application/json", data="<request>")]
fn create_account(request: JSON<CreateAccountRequest>,
                  conn: db::Conn) -> Result<JSON<Account>, Error> {
    let new_account = NewAccount {
        username: &request.username,
        balance: request.balance,
    };

    // Make sure to create the account in a transaction in case there's a failure.
    conn.transaction(|| {
        let account = Account::create_from(&conn, new_account)?;
        Ok(JSON(account))
    })
}

#[post("/<username>/transfer", format="application/json", data="<request>")]
fn transfer(username: String,
            request: JSON<TransferRequest>,
            conn: db::Conn) -> Result<JSON<Value>, Error> {

    // Make sure to wrap the transfer in a transaction in case there's a failure.
    conn.transaction(|| {
        let mut src_account = Account::find_by_username(&conn, &username)?;
        let mut dst_account = Account::find_by_username(&conn, &request.username)?;

        src_account.transfer(&conn, &mut dst_account, request.amount)?;

        Ok(JSON(json!({
            "msg": "transfer completed",
        })))
    })
}

fn main() {
    let pool = quad_diesel::establish_connection_pool("../db.sqlite")
        .expect("failed to connect to the database");

    rocket::ignite()
        .manage(pool)
        .mount("/", routes![show_accounts, show_account, create_account, transfer])
        .launch();
}

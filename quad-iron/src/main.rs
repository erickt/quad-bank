#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate bodyparser;
extern crate diesel;
extern crate iron;
extern crate logger;
extern crate persistent;
extern crate quad_diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate router;
extern crate serde;
extern crate simplelog;

use diesel::Connection;
use iron::headers::ContentType;
use iron::prelude::*;
use iron::status;
use iron::typemap::Key;
use logger::Logger;
use persistent::Read;
use quad_diesel::models::{Account, NewAccount};
use router::Router;
use serde::Serialize;

/// Create an adaptor that allows us to extract a database connection from our request.
pub struct DB;
impl Key for DB {
    type Value = quad_diesel::SqliteConnectionPool;
}

/// The body request for creating an account.
#[derive(Clone, Deserialize)]
struct CreateAccountRequest {
    /// The name of the account to create.
    username: String,

    /// How many quadbucks to put in this account.
    balance: i32,
}

/// The http body request for transfering quadbucks to an account.
#[derive(Clone, Deserialize)]
struct TransferRequest {
    /// The recipient account of the quadbucks.
    username: String,

    /// How many quadbucks should we give to this account?
    amount: i32,
}

/// Serialize a type into a json response.
fn json_response<T: Serialize>(status: status::Status, value: &T) -> IronResult<Response> {
    let response = serde_json::to_string(value).unwrap();
    Ok(Response::with((ContentType::json().0, status, response)))
}

fn show_accounts(req: &mut Request) -> IronResult<Response> {
    let conn = req.get::<Read<DB>>().unwrap().as_ref().get().unwrap();

    let accounts = Account::all(&conn).unwrap();

    json_response(status::Ok, &accounts)
}

fn show_account(req: &mut Request) -> IronResult<Response> {
    let conn = req.get::<Read<DB>>().unwrap().as_ref().get().unwrap();

    let router = req.extensions.get::<Router>().unwrap();
    let username = router.find("username").unwrap();

    let account = Account::find_by_username(&conn, &username).unwrap();

    json_response(status::Ok, &account)
}

fn create_account(req: &mut Request) -> IronResult<Response> {
    let conn = &*req.get::<Read<DB>>().unwrap().as_ref().get().unwrap();

    let request = req.get::<bodyparser::Struct<CreateAccountRequest>>()
        .unwrap()
        .unwrap();

    let new_account = NewAccount {
        username: &request.username,
        balance: request.balance,
    };

    // Create the account in a transaction.
    let account = conn.transaction(|| Account::create_from(&conn, new_account))
        .expect("failed to create account");

    json_response(status::Created, &account)
}

fn transfer(req: &mut Request) -> IronResult<Response> {
    let conn = req.get::<Read<DB>>().unwrap().as_ref().get().unwrap();

    let request = req.get::<bodyparser::Struct<TransferRequest>>()
        .unwrap()
        .unwrap();

    let router = req.extensions.get::<Router>().unwrap();
    let username = router.find("username").unwrap();

    // Transfer the quadbucks in a transaction.
    conn.transaction(|| {
        let mut src_account = Account::find_by_username(&conn, &username)?;
        let mut dst_account = Account::find_by_username(&conn, &request.username)?;

        src_account.transfer(&conn, &mut dst_account, request.amount)
    }).expect("failed to transfer quadbucks");

    json_response(status::Ok, &json!({ "msg": "transfer completed" }))
}

fn main() {
    // Initialize the logger to capture requests made in the route handlers.
    simplelog::SimpleLogger::init(simplelog::LogLevelFilter::Info,
                                  simplelog::Config::default())
        .unwrap();

    // Create a router that directs urls to different handlers.
    let mut router = Router::new();
    router.get("/", show_accounts, "show_accounts");
    router.post("/", create_account, "create_account");
    router.get("/:username", show_account, "show_account");
    router.post("/:username/transfer", transfer, "transfer");

    // Wrap the router middleware in a logger middleware to emit any log messages.
    let (logger_before, logger_after) = Logger::new(None);
    let mut chain = Chain::new(router);

    chain.link_before(logger_before);

    // Connect to the database and add it into the state.
    let pool = quad_diesel::establish_connection_pool("../db.sqlite")
        .expect("failed to connect to database");
    chain.link(Read::<DB>::both(pool));

    chain.link_after(logger_after);

    let address = "localhost:8000";
    println!("listening on {}", address);

    // Finally, launch the server.
    Iron::new(chain)
        .http(address)
        .unwrap();
}

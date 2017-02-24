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

mod db;

use iron::headers::ContentType;
use iron::prelude::*;
use iron::status;
use logger::Logger;
use persistent::Read;
use router::Router;

#[derive(Clone, Deserialize)]
struct CreateAccountRequest {
    username: String,
    balance: i32,
}

#[derive(Clone, Deserialize)]
struct TransferRequest {
    dst_username: String,
    amount: i32,
}

fn show_accounts(req: &mut Request) -> IronResult<Response> {
    let conn = req.get::<Read<db::DB>>().unwrap().as_ref().get().unwrap();

    let accounts = quad_diesel::all_accounts(&conn).unwrap();
    let response = serde_json::to_string(&accounts).unwrap();

    Ok(Response::with((ContentType::json().0, status::Ok, response)))
}

fn show_account(req: &mut Request) -> IronResult<Response> {
    let conn = req.get::<Read<db::DB>>().unwrap().as_ref().get().unwrap();

    let router = req.extensions.get::<Router>().unwrap();
    let username = router.find("username").unwrap();

    let account = quad_diesel::account_for_username(&conn, &username).unwrap();
    let response = serde_json::to_string(&account).unwrap();

    Ok(Response::with((ContentType::json().0, status::Ok, response)))
}

fn create_account(req: &mut Request) -> IronResult<Response> {
    let conn = req.get::<Read<db::DB>>().unwrap().as_ref().get().unwrap();

    let request = req.get::<bodyparser::Struct<CreateAccountRequest>>()
        .unwrap()
        .unwrap();

    let account = quad_diesel::create_account(
        &conn,
        &request.username,
        request.balance,
    ).unwrap();
    let response = serde_json::to_string(&account).unwrap();

    Ok(Response::with((ContentType::json().0, status::Ok, response)))
}

fn transfer(req: &mut Request) -> IronResult<Response> {
    let conn = req.get::<Read<db::DB>>().unwrap().as_ref().get().unwrap();

    let request = req.get::<bodyparser::Struct<TransferRequest>>()
        .unwrap()
        .unwrap();

    let router = req.extensions.get::<Router>().unwrap();
    let username = router.find("username").unwrap();

    quad_diesel::transfer(
        &conn,
        &username,
        &request.dst_username,
        request.amount,
    ).unwrap();

    let response = serde_json::to_string(&json!({
        "msg": "transfer completed"
    })).unwrap();

    Ok(Response::with((ContentType::json().0, status::Ok, response)))
}

fn main() {
    // Initialize the logger to capture requests made in the route handlers.
    simplelog::SimpleLogger::init(
        simplelog::LogLevelFilter::Info,
        simplelog::Config::default()
    ).unwrap();

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
    let pool = db::init_pool("../db.sqlite");
    chain.link(Read::<db::DB>::both(pool));

    chain.link_after(logger_after);

    // Finally, launch the server.
    Iron::new(chain)
        .http("localhost:8000")
        .unwrap();
}

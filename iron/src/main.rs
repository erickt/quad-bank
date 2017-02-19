extern crate bodyparser;
extern crate iron;
extern crate logger;
extern crate persistent;
extern crate router;
extern crate serde_json;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate slog;
extern crate slog_scope;
extern crate slog_scope_stdlog;
extern crate slog_term;

mod log;
mod errors;
mod handlers;
mod server;

use iron::prelude::*;
use logger::Logger;
use persistent::State;
use router::Router;
use slog::DrainExt;
use std::process;

fn build_logger() -> slog::Logger {
    let drain = slog::level_filter(slog::Level::Info, slog_term::streamer().build().fuse());
    let logger = slog::Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION")));

    slog_scope::set_global_logger(logger.clone());
    slog_scope_stdlog::init().unwrap();

    logger
}

fn main() {
    let root_logger = build_logger();

    let mut router = Router::new();
    router.post("/", handlers::submit_payment, "submit_payment");

    let mut chain = Chain::new(router);

    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_before(State::<log::IronLogger>::one(root_logger.clone()));
    chain.link_after(logger_after);

    match Iron::new(chain).http("localhost:3000") {
        Ok(listening) => {
            info!(root_logger, "Server is running on {}", listening.socket);
        }
        Err(err) => {
            error!(root_logger, "error: {}", err);
            process::exit(1);
        }
    }
}

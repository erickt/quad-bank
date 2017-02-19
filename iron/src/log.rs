// inspired by https://github.com/exul/matrix-rocketchat

use iron::{Plugin, Request};
use iron::typemap::Key;
use persistent::State;
use slog::Logger;

use errors::*;

pub struct IronLogger;

impl IronLogger {
    pub fn from_request(request: &mut Request) -> Result<Logger> {
        let lock = request.get::<State<IronLogger>>().chain_err(|| {
            ErrorKind::LoggerExtractionError
        })?;
        let logger = lock.read().expect("lock was poisoned");
        Ok(logger.clone())
    }
}

impl Key for IronLogger {
    type Value = Logger;
}

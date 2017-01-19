use iron::prelude::*;
use iron::status;

use bodyparser;
use log;

pub fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World")))
}

pub fn submit_payment(request: &mut Request) -> IronResult<Response> {
    let logger = log::IronLogger::from_request(request)?;

    let body = match request.get::<bodyparser::Json>() {
        Ok(Some(body)) => body,
        Ok(None) => {
            return Ok(Response::with(status::BadRequest));
        }
        Err(err) => {
            error!(logger, "{}", err);
            return Ok(Response::with(status::BadRequest));
        }
    };

    info!(logger, "{:#?}", body);

    Ok(Response::with(status::Ok))
}

// Inspired by https://github.com/exul/matrix-rocketchat

use iron::{IronError, Response};
use iron::modifier::Modifier;
use iron::status::Status;
use serde_json;

error_chain! {
    errors {
        InvalidJSON(msg: String) {
            description("The provided JSON is not valid.")
            display("Could not process request, the submitted data is not valid JSON: {}", msg)
        }

        LoggerExtractionError{
            description("Getting logger from iron request failed")
            display("Getting logger from iron request failed")
        }
    }
}

impl ErrorKind {
    pub fn status_code(&self) -> Status {
        match *self {
            ErrorKind::InvalidJSON(_) => Status::UnprocessableEntity,
            _ => Status::InternalServerError,
        }
    }
}

impl From<Error> for IronError {
    fn from(error: Error) -> IronError {
        let response = Response::with(&error);
        IronError {
            error: Box::new(error),
            response: response,
        }
    }
}

impl<'a> Modifier<Response> for &'a Error {
    fn modify(self, response: &mut Response) {
        let resp = serde_json::builder::ObjectBuilder::new()
            .insert("error", format!("{}", self))
            .insert_array("causes", |mut builder| {
                for err in self.iter() {
                    builder = builder.push(format!("{}", err));
                };

                builder
            })
            .build();

        let err_msg = serde_json::to_string(&resp).expect("ErrorResponse is always serializable");
        response.status = Some(self.status_code());
        response.body = Some(Box::new(err_msg));
    }
}

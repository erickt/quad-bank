use diesel::result::DatabaseErrorKind;
use diesel::result::Error as ResultError;
use rocket::http::Status;
use rocket::response::{Response, Responder};
use std::fmt;

pub enum Error {
    NotFound,
    Conflict,
    InternalServerError,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NotFound => f.write_str("Not Found"),
            Error::Conflict => f.write_str("Conflict"),
            Error::InternalServerError => f.write_str("Internal Server Error"),
        }
    }
}

impl From<ResultError> for Error {
    fn from(err: ResultError) -> Self {
        match err {
            ResultError::NotFound => Error::NotFound,
            ResultError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => Error::Conflict,
            _ => Error::InternalServerError,
        }
    }
}

impl<'r> Responder<'r> for Error {
    fn respond(self) -> ::std::result::Result<Response<'r>, Status> {
        match self {
            Error::NotFound => Err(Status::NotFound),
            Error::Conflict => Err(Status::Conflict),
            Error::InternalServerError => Err(Status::InternalServerError),
        }
    }
}

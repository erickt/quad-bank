#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate serde_derive;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde;

pub mod models;
pub mod schema;

use diesel::sqlite::SqliteConnection;
use diesel::{Connection, ConnectionError};

pub type SqliteConnectionPool = r2d2::Pool<r2d2_diesel::ConnectionManager<SqliteConnection>>;

/// Create a connection to the SQLite database.
pub fn establish_connection(database_url: &str) -> Result<SqliteConnection, ConnectionError> {
    SqliteConnection::establish(database_url)
}

/// Create a pool of SQLite connections to the database.
pub fn establish_connection_pool(database_url: &str)
                                 -> Result<SqliteConnectionPool, r2d2::InitializationError> {
    let config = r2d2::Config::default();
    let manager = r2d2_diesel::ConnectionManager::new(database_url);
    r2d2::Pool::new(config, manager)
}

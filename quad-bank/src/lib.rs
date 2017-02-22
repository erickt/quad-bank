#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate serde_derive;
extern crate serde;

pub mod models;
pub mod schema;

use diesel::Connection;
use diesel::prelude::*;
use diesel::result::Error as ResultError;
use diesel::sqlite::SqliteConnection;

use models::{Account, NewAccount};
use schema::accounts;

pub fn establish_connection(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(database_url)
        .expect(&format!("failed to connect to database {}", database_url))
}

pub fn all_accounts(conn: &SqliteConnection) -> Result<Vec<Account>, ResultError> {
    accounts::table.load::<Account>(conn)
}

pub fn account_for_username(conn: &SqliteConnection,
                            username: &str) -> Result<Account, ResultError> {
    accounts::table
        .filter(accounts::username.eq(username))
        .first::<Account>(conn)
}

pub fn create_account(conn: &SqliteConnection,
                      username: &str,
                      balance: i32) -> Result<Account, ResultError> {
    let new_account = NewAccount {
        username: username,
        balance: balance,
    };

    conn.transaction(|| {
        diesel::insert(&new_account).into(accounts::table)
            .execute(conn)?;

        let account = accounts::table
            .order(accounts::id.desc())
            .first(conn)?;

        Ok(account)
    })
}

pub fn transfer(conn: &SqliteConnection,
                src_username: &str,
                dst_username: &str,
                amount: i32) -> Result<(), ResultError> {
    use schema::accounts::dsl::*;

    conn.transaction(|| {
        let mut src_account = accounts
            .filter(username.eq(src_username))
            .first::<Account>(conn)?;

        let mut dst_account = accounts
            .filter(username.eq(dst_username))
            .first::<Account>(conn)?;

        src_account.balance -= amount;
        dst_account.balance += amount;

        src_account.save_changes::<Account>(conn)?;
        dst_account.save_changes::<Account>(conn)?;

        Ok(())
    })
}

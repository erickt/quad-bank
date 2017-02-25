use diesel::prelude::*;
use diesel::result::Error as ResultError;
use diesel::sqlite::SqliteConnection;
use diesel;

use super::schema::accounts;

/// The account structure.
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
pub struct Account {
    /// The unique identifier of the account.
    pub id: i32,

    /// The unique username of the account.
    pub username: String,

    /// The amount of quadbucks in the account.
    pub balance: i32,
}

impl Account {
    /// Return all the accounts in the database.
    pub fn all(conn: &SqliteConnection) -> Result<Vec<Account>, ResultError> {
        accounts::table.load(conn)
    }

    /// Find an `Account` in the database by the username.
    pub fn find_by_username(conn: &SqliteConnection,
                            username: &str)
                            -> Result<Account, ResultError> {
        accounts::table.filter(accounts::username.eq(username))
            .first(conn)
    }

    /// Create an `Account` from a `NewAccount`. Note this might error with a unique constraint
    /// error if the account with this username already exists.
    pub fn create_from(conn: &SqliteConnection, new_account: NewAccount)
                       -> Result<Account, ResultError> {
        diesel::insert(&new_account).into(accounts::table)
            .execute(conn)?;

        let account = accounts::table.order(accounts::id.desc())
            .first(conn)?;

        Ok(account)
    }

    /// Save all changes back to the database.
    pub fn save(&mut self, conn: &SqliteConnection) -> Result<(), ResultError> {
        self.save_changes::<Account>(conn)?;
        Ok(())
    }

    /// Transfer some `amount` quadbucks from this account to another account.
    pub fn transfer(&mut self,
                    conn: &SqliteConnection,
                    other: &mut Account,
                    amount: i32)
                    -> Result<(), ResultError> {
        self.balance -= amount;
        other.balance += amount;

        self.save(conn)?;
        other.save(conn)?;

        Ok(())
    }
}

#[derive(Insertable)]
#[table_name="accounts"]
pub struct NewAccount<'a> {
    pub username: &'a str,
    pub balance: i32,
}

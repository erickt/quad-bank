extern crate clap;
extern crate diesel;
extern crate quad_diesel;

use clap::{Arg, App, SubCommand};
use diesel::Connection;
use quad_diesel::models::{Account, NewAccount};

fn main() {
    // We use clap to create a simple commandline interface. This will parse out the following
    // commands:
    //
    //   admin --database ../db.sqlite show-accounts
    //   admin --database ../db.sqlite create-account erickt 100
    //   admin --database ../db.sqlite transfer erickt carols10cents 10
    let matches = App::new("quad-admin")
        .arg(Arg::with_name("database")
            .short("d")
            .long("database")
            .value_name("DATABASE")
            .takes_value(true)
            .required(true)
            .help("the path to the sqlite database"))
        .subcommand(SubCommand::with_name("show-accounts").about("show all accounts in diesel"))
        .subcommand(SubCommand::with_name("create-account")
            .about("Creates a new account")
            .arg(Arg::with_name("username")
                .takes_value(true)
                .required(true)
                .help("the account's username"))
            .arg(Arg::with_name("balance")
                .takes_value(true)
                .required(true)
                .help("the account's balance")))
        .subcommand(SubCommand::with_name("transfer")
            .about("Transfers balance between two accounts")
            .arg(Arg::with_name("src_username")
                .takes_value(true)
                .required(true)
                .help("the source username of the transfer"))
            .arg(Arg::with_name("dst_username")
                .takes_value(true)
                .required(true)
                .help("the destination username of the transfer"))
            .arg(Arg::with_name("amount")
                .takes_value(true)
                .required(true)
                .help("the amount to transfer into the account")))
        .get_matches();

    // Now that we parsed our commandline arguments, lets connect to the database.
    let database_url = matches.value_of("database")
        .expect("database not found");
    let conn = quad_diesel::establish_connection(&database_url)
        .expect("failed to connect to database");

    match matches.subcommand() {
        // Print out all the accounts in the database.
        ("show-accounts", Some(_)) => {
            let results = Account::all(&conn)
                .expect("failed to load accounts");

            println!("Displaying {} accounts", results.len());
            for account in results {
                println!("{}\t{}\t{}", account.id, account.username, account.balance);
            }
        }

        // Create an account.
        ("create-account", Some(matches)) => {
            // Parse out the command arguments.
            let username = matches.value_of("username")
                .expect("username was not present");
            let balance = matches.value_of("balance")
                .expect("balance argument was not present")
                .parse::<i32>()
                .expect("balance was not an integer");

            // Now, actually create the account.
            let account = NewAccount {
                username: username,
                balance: balance,
            };

            conn.transaction(|| {
                Account::create_from(&conn, account)
            }).expect("failed create account");

            println!("account created");
        }

        // Transfer money between accounts.
        ("transfer", Some(matches)) => {
            let src_username = matches.value_of("src_username")
                .expect("src_username was not present");
            let dst_username = matches.value_of("dst_username")
                .expect("dst_username was not present");
            let amount = matches.value_of("amount")
                .expect("amount was not present")
                .parse::<i32>()
                .expect("amount to transfer is not an integer");

            // Make sure to wrap the transfer in a transaction in case there's a failure.
            conn.transaction(|| {
                let mut src_account = Account::find_by_username(&conn, src_username)?;
                let mut dst_account = Account::find_by_username(&conn, dst_username)?;

                src_account.transfer(&conn, &mut dst_account, amount)
            })
            .expect("failed to transfer balance");

            println!("transfer completed");
        }
        ("", None) => {
            // This gets called when there's no subcommand specified. We'll just exit.
        }
        _ => {
            panic!("this should have been unreachable")
        }
    }
}

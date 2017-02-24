extern crate clap;
extern crate diesel;
extern crate quad_bank;

use clap::{Arg, App, SubCommand};
use diesel::prelude::*;
use quad_bank::models::*;
use quad_bank::schema::accounts;

fn main() {
    let matches = App::new("quad-admin")
        .arg(Arg::with_name("database")
             .short("d")
             .long("database")
             .value_name("DATABASE")
             .takes_value(true)
             .required(true)
             .help("the path to the sqlite database"))
        .subcommand(
            SubCommand::with_name("show-accounts")
                .about("show all accounts in bank")
        )
        .subcommand(
            SubCommand::with_name("create-account")
                .about("Creates a new account")
                .arg(Arg::with_name("username")
                     .takes_value(true)
                     .required(true)
                     .help("the account's username"))
                .arg(Arg::with_name("balance")
                     .takes_value(true)
                     .required(true)
                     .help("the account's balance"))
        )
        .subcommand(
            SubCommand::with_name("transfer")
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
                     .help("the amount to transfer into the account"))
        )
        .get_matches();

    let database_url = matches.value_of("database")
        .expect("database not found");
    let conn = quad_bank::establish_connection(&database_url);

    match matches.subcommand() {
        ("show-accounts", Some(_)) => {
            let results = accounts::table.load::<Account>(&conn)
                .expect("failed to load accounts");

            println!("Displaying {} accounts", results.len());
            for account in results {
                println!("{}\t{}\t{}", account.id, account.username, account.balance);
            }
        }
        ("create-account", Some(matches)) => {
            let username = matches.value_of("username")
                .expect("username was not present");
            let balance = matches.value_of("balance")
                .expect("balance argument was not present")
                .parse::<i32>()
                .expect("balance was not an integer");

            quad_bank::create_account(&conn, username, balance)
                .expect("failed create account");
            println!("account created");
        }
        ("transfer", Some(matches)) => {
            let src_username = matches.value_of("src_username")
                .expect("src_username was not present");
            let dst_username = matches.value_of("dst_username")
                .expect("dst_username was not present");
            let amount = matches.value_of("amount")
                .expect("amount was not present")
                .parse::<i32>()
                .expect("amount to transfer is not an integer");

            quad_bank::transfer(&conn, src_username, dst_username, amount)
                .expect("failed to transfer balance");
            println!("transfer completed");
        }
        ("", None) => {}
        _ => { unreachable!() }
    }
}

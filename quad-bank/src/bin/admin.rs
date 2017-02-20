#[macro_use] extern crate clap;
#[macro_use] extern crate error_chain;
extern crate diesel;
extern crate quad_bank;

use clap::{Arg, App, SubCommand};
use diesel::prelude::*;
use quad_bank::models::*;
use quad_bank::schema::accounts;
use std::io::{self, Write};
use std::process;

error_chain! {
    foreign_links {
        DieselConnectionError(diesel::ConnectionError);
        DieselResultError(diesel::result::Error);
        ClapError(clap::Error);
    }

    errors {
        NoCommandSpecified {
            description("no command specified")
        }
    }
}

fn main() {
    if let Err(err) = run() {
        let mut stderr = io::stderr();
        writeln!(&mut stderr, "error: {}", err).unwrap();
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let app = App::new("quad-admin");
    let matches = app
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(SubCommand::with_name("show-accounts"))
        .subcommand(
            SubCommand::with_name("create-account")
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

    let conn = quad_bank::establish_connection()?;

    match matches.subcommand() {
        ("show-accounts", Some(_)) => {
            let results = accounts::table.load::<Account>(&conn)?;

            println!("Displaying {} accounts", results.len());
            for account in results {
                println!("{}\t{}\t{}", account.id, account.username, account.balance);
            }

            Ok(())
        }
        ("create-account", Some(matches)) => {
            let username = matches.value_of("username").unwrap();
            let balance = value_t!(matches, "balance", i32).unwrap();

            quad_bank::create_account(&conn, username, balance)?;
            println!("account created");

            Ok(())
        }
        ("transfer", Some(matches)) => {
            let src_username = matches.value_of("src_username").unwrap();
            let dst_username = matches.value_of("dst_username").unwrap();
            let amount = value_t!(matches, "amount", i32).unwrap();

            quad_bank::transfer(&conn, src_username, dst_username, amount)?;
            println!("transfer completed");

            Ok(())
        }
        ("", None) => {
            bail!(ErrorKind::NoCommandSpecified)
        }
        _ => { unreachable!() }
    }
}

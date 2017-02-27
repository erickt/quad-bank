# Overhanded Quadrilateral Bank

[![Travis Build Status](https://travis-ci.org/erickt/quad-bank.png?branch=master)](https://travis-ci.org/erickt/quad-bank)

[Documentation](https://erickt.github.io/quad-bank)

`quad_bank` is a Rust project to demonstrate how one can build a payment
processor for the [Underhanded Rust
Contest](https://underhanded.rs/blog/2016/12/15/underhanded-rust.en-US.html).
Feel free to use this as a reference to help you get started, but don't feel
obligated to base your code on this project.

This project uses a few popular Rust technologies to show how to interact with
the bank either through the command line, or through some REST web requests:

* [Diesel](http://diesel.rs), a high level database ORM
* [Clap](https://github.com/kbknapp/clap-rs), a command line parser
* [Iron](http://ironframework.io), an extensible web framework
* [Rocket](https://rocket.rs), a Rust-nightly-only web framework that cuts out
  boilerplate code with experimental Rust extensions.

The code is (hopefully) pretty straightforward, but we need to first do a
little setup before you can play with quad-bank.

# Installing Rust and building the examples

First thing you need to do is [install
Rust](https://www.rust-lang.org/en-US/install.html). Our documentation here
will use the [rustup](https://github.com/rust-lang-nursery/rustup.rs) tool.
We'll install both the Stable and Nightly versions of Rust so we can run the
rocket example:

```
% rustup install stable
% rustup install nightly
```

To build all the projects, just run this:

```
% (cd quad-diesel && rustup run stable cargo build)
% (cd quad-cli && rustup run stable cargo build)
% (cd quad-iron && rustup run stable cargo build)
% (cd quad-rocket && rustup run nightly cargo build)
```

# Setting up the database

After we're all built, we need to create our database. We'll using
[SQLite](https://www.sqlite.org/), a simple local database, along with
[diesel\_cli](https://crates.io/crates/diesel_cli), to create our tables.
`diesel_cli` uses [migration
scripts](http://docs.diesel.rs/diesel/migrations/index.html), to drive changing
the schema. You can see the ones we've created in the
[migrations](https://github.com/erickt/quad-bank/tree/master/migrations)
directory. As of this writing, there's just one migration that has
[up.sql](https://github.com/erickt/quad-bank/blob/master/migrations/20170219173610_create_accounts/up.sql),
that creates our `accounts` table, and
[down.sql](https://github.com/erickt/quad-bank/blob/master/migrations/20170219173610_create_accounts/down.sql),
that deletes the table if we roll back this migration.

We have wrapped up installing `diesel_cli` and running the migrations in the
`bootstrap.sh` script:

```
% ./bootstrap.sh
Installing the diesel CLI
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading mysqlclient-sys v0.2.1
 Downloading pq-sys v0.4.2
...
   Compiling diesel_cli v0.11.0
    Finished release [optimized] target(s) in 52.46 secs
  Installing /Users/erickt/.cargo/bin/diesel
Running the migrations...
```

This will create a `db.sqlite` file in the top level directory of the project.

# Running quad-cli

Lets start off with `quad_cli`. This is a higher level interface for
interacting with our database. It supports three subcommands:

* `show-accounts`, which just lists all the accounts
* `create-account`, which creates an account
* `transfer`, which transfers quadbucks between accounts

With a fresh database, there are no accounts to see (note that the `-q` just
suppresses the cargo output, and '--' passes the rest of the arguments on to
`quad-cli`):

```
% cd quad-cli
% rustup run stable cargo run -q -- show-accounts
Displaying 0 accounts
```

So lets create two:

```
% rustup run stable cargo run -q -- create-account erickt 100
account created
% rustup run stable cargo run -q -- create-account carols10cents 100
account created
```

Now we have two accounts:

```
% rustup run stable cargo run -q -- show-accounts
Displaying 2 accounts
1	erickt	100
2	carols10cents	100
```

And lets give Carol some money:

```
% rustup run stable cargo run -q -- transfer erickt carols10cents 10
transfer completed
% rustup run stable cargo run -q -- show-accounts
Displaying 2 accounts
1	erickt	90
2	carols10cents	110
```

Pretty straightforward. For fun, `quad-bank` has probably the simplest exploit.
We've created the database with signed integers, and conveniently forgot to add
any bounds checking, so it's very simple to go into massive amounts of debt:

```
% rustup run stable cargo run -q -- transfer erickt carols10cents 1000000
transfer completed
% rustup run stable cargo run -q -- show-accounts
Displaying 2 accounts
1	erickt	-999910
2	carols10cents	1000110
```

Congrats [carols10cents](http://github.com/carols10cents), you're rich! Your
humble author, however, is sadly destined to debtors prison. Fortunately
though, it's quite easy to get my money back:

```
% rustup run stable cargo run -q -- transfer erickt carols10cents -- -1000000
transfer completed
% rustup run stable cargo run -q -- show-accounts
Displaying 2 accounts
1	erickt	90
2	carols10cents	110
```

# Running quad-iron and quad-rocket

Now for the web services. They both implement the same REST API:

* GET / - get all the accounts
* GET /erickt - `erickt`'s account information
* POST / - create an account
* POST /erickt/transfer - transfer money between accounts

To run `quad-iron`, run:

```
% rustup run stable cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/quad-iron`
listening on localhost:8000
```

To run `quad-rocket`, run:

```
% rustup run nightly-2017-02-12 cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/quad-rocket`
🔧  Configured for development.
    => address: localhost
    => port: 8000
    => log: normal
    => workers: 8
🛰  Mounting '/':
    => GET /
    => GET /<username>
    => POST / application/json
    => POST /<username>/transfer application/json
🚀  Rocket has launched from http://localhost:8000...
```

We'll use [httpie](https://httpie.org/) to interact with the server, and we can
see the accounts we made before.

```
% http GET localhost:8000
HTTP/1.1 200 OK
Content-Length: 93
Content-Type: application/json
Date: Sun, 26 Feb 2017 21:24:32 GMT

[
    {
        "balance": 90,
        "id": 1,
        "username": "erickt"
    },
    {
        "balance": 110,
        "id": 2,
        "username": "carols10cents"
    }
]
```

Or we can look at a single user's account:

```
% http GET localhost:8000/erickt
HTTP/1.1 200 OK
Content-Length: 41
Content-Type: application/json
Date: Sun, 26 Feb 2017 21:25:29 GMT

{
    "balance": 90,
    "id": 1,
    "username": "erickt"
}
```

Creating an account for [Manishearth](https://github.com/Manishearth) is done
with this:

```
% http -j POST localhost:8000 username=Manishearth balance=2000
HTTP/1.1 200 OK
Content-Length: 48
Content-Type: application/json
Date: Sun, 26 Feb 2017 21:28:52 GMT
Server: rocket

{
    "balance": 2000,
    "id": 3,
    "username": "Manishearth"
}
```

And finally, [carols10cents](http://github.com/carols10cents) can rob him blind
with:

```
% http POST localhost:8000/Manishearth/transfer amount=2000 username=carols10cents
HTTP/1.1 200 OK
Content-Length: 28
Content-Type: application/json
Date: Mon, 26 Feb 2017 21:29:09 GMT
Server: rocket

{
    "msg": "transfer completed"
}
```

Manishearth has no quadbucks left:

```
% http GET localhost:8000
HTTP/1.1 200 OK
Content-Length: 140
Content-Type: application/json
Date: Sun, 26 Feb 2017 21:30:21 GMT
Server: rocket

[
    {
        "balance": 90,
        "id": 1,
        "username": "erickt"
    },
    {
        "balance": 2110,
        "id": 2,
        "username": "carols10cents"
    },
    {
        "balance": 0,
        "id": 3,
        "username": "Manishearth"
    }
]
```

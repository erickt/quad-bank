CREATE TABLE accounts (
  id INTEGER PRIMARY KEY NOT NULL,
	username VARCHAR NOT NULL,
	balance INTEGER NOT NULL
);
CREATE UNIQUE INDEX account_username ON accounts(username);

#!/bin/bash -eu

cd `dirname $0`

export PATH=$PATH:/home/travis/.cargo/bin

# Install diesel CLI if they're not installed.
if ! command -v diesel >/dev/null 2>&1; then
	echo Installing the diesel CLI
	cargo install diesel_cli
fi

echo Running the migrations...

# Create the database.
diesel migration run \
	--database-url db.sqlite \
	--migration-dir quad-diesel/migrations

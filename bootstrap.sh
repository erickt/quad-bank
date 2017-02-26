#!/bin/bash -eu

cd `dirname $0`

export PATH=$HOME/.local/bin:$PATH
export DATABASE_URL=db.sqlite

# Install diesel CLI if they're not installed.
if ! command -v diesel >/dev/null 2>&1; then
	echo Installing the diesel CLI
	cargo install diesel_cli
fi

echo Running the migrations...

# Create the database.
diesel migration run

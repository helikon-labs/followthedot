#!/usr/bin/env bash
set -e

cd "${0%/*}" || exit # cd script directory
PGPASSWORD=postgres psql -h 127.0.0.1 -U postgres -c "DROP DATABASE IF EXISTS ftd";
PGPASSWORD=postgres psql -h 127.0.0.1 -U postgres -tc "SELECT 1 FROM pg_user WHERE usename = 'ftd'" | grep -q 1 ||  PGPASSWORD=postgres psql -h 127.0.0.1 -U postgres -c "CREATE USER ftd WITH ENCRYPTED PASSWORD 'ftd';"
PGPASSWORD=postgres psql -h 127.0.0.1 -U postgres -c "CREATE DATABASE ftd;"
PGPASSWORD=postgres psql -h 127.0.0.1 -U postgres -c "GRANT ALL PRIVILEGES ON DATABASE ftd TO ftd;"
PGPASSWORD=postgres psql -h 127.0.0.1 -U postgres -c "ALTER DATABASE ftd OWNER TO ftd;"
DATABASE_URL=postgres://ftd:ftd@127.0.0.1/ftd sqlx migrate run

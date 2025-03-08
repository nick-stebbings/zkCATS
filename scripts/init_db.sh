#!/usr/bin/env bash

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi
if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 " cargo install --version='~0.7' sqlx-cli \
--no-default-features --features rustls,postgres"
  echo >&2 "to install it."
  exit 1
fi

# The rest of the script
set -x
set -eo pipefail

# Default settings
DB_HOST="${DB_HOST:=localhost}"
DB_PORT="${DB_PORT:=5432}"

  SUPERUSER="${SUPERUSER:=postgres}"
SUPERUSER_PWD="${SUPERUSER_PWD:=welcome}"
DB_USER="${DB_USER:=postgres}"
DB_PASSWORD="${DB_PASSWORD:=welcome}"

DB_NAME="${DB_NAME:=postgres}"


# Allow to skip Docker if a dockerized Postgres database is already running
if [[ -z "${SKIP_DOCKER}" ]]
then
  # if a postgres container is running, print instructions to kill it and exit
  RUNNING_POSTGRES_CONTAINER=$(docker ps --filter 'name=pg' --format '{{.ID}}')
  if [[ -n $RUNNING_POSTGRES_CONTAINER ]]; then
    echo >&2 "there is a postgres container already running, kill it with"
    echo >&2 "    docker stop ${RUNNING_POSTGRES_CONTAINER}"
    exit 1
  fi
  CONTAINER_NAME="pg"
  # Launch postgres using Docker
  docker run \
      --env POSTGRES_USER=${SUPERUSER} \
      --env POSTGRES_PASSWORD=${SUPERUSER_PWD} \
      --health-cmd="pg_isready -U ${SUPERUSER} || exit 1" \
      --health-interval=1s \
      --health-timeout=5s \
      --health-retries=5 \
      --publish "${DB_PORT}":5432 \
      --detach \
      --name "${CONTAINER_NAME}" \
      postgres -N 1000
      # ^ Increased maximum number of connections for testing purposes

  until [ \
    "$(docker inspect -f "{{.State.Health.Status}}" ${CONTAINER_NAME})" == \
    "healthy" \
  ]; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
  done

  DB_USER="${SUPERUSER}"
  DB_PASSWORD="${SUPERUSER_PWD}"
  DB_NAME="${DB_NAME}"
else
  # We're skipping docker, so we need to ensure the DB variables are set
  # In CI, these should match the GitHub Actions postgres service settings
  echo "Using existing PostgreSQL instance at ${DB_HOST}:${DB_PORT}"
fi

# Debug info - print connection details
echo "Connecting with: host=${DB_HOST}, port=${DB_PORT}, user=${DB_USER}, db=${DB_NAME}"

# Keep pinging Postgres until it's ready to accept commands
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 2
done
>&2 echo "Postgres is up and running on port ${DB_PORT}!"


DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
MIGRATIONS_PATH="sql/dev_initial"

# Run migrations
if [ -d "${MIGRATIONS_PATH}" ]; then
  echo "Running migrations from ${MIGRATIONS_PATH}..."
  sqlx migrate run --source "${MIGRATIONS_PATH}" --database-url "${DATABASE_URL}"
  echo "Migrations complete!"
else
  echo "Error: Could not find migrations directory. Please set MIGRATIONS_PATH to the correct location."
  exit 1
fi

>&2 echo "Postgres has been migrated, ready to go!"
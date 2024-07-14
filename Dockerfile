FROM rust:alpine

COPY migrations /build/migrations
COPY src /build/src
COPY build.rs /build/build.rs
COPY Cargo.lock /build/Cargo.lock
COPY Cargo.toml /build/Cargo.toml
COPY .env /build/.env

WORKDIR /build

RUN source .env && \
    apk update && \
    apk upgrade --no-cache && \
    apk add --no-cache build-base libc-dev postgresql pkgconf openssl-dev postgresql-contrib && \
    su - postgres -c "initdb -D /var/lib/postgresql/data" && \
    mkdir -p /run/postgresql && \
    chown postgres:postgres /run/postgresql && \
    chmod 775 /run/postgresql && \
    su - postgres -c "pg_ctl -D /var/lib/postgresql/data start" && \
    su - postgres -c "psql -c \"CREATE USER $DATABASE_USER WITH PASSWORD '$DATEBASE_PASS';\"" && \
    su - postgres -c "psql -c \"CREATE DATABASE $DATABASE_NAME OWNER $DATABASE_USER;\"" && \
    cargo install sqlx-cli --no-default-features --features postgres && \
    sqlx migrate run && \
    cargo build --release && \
    mkdir /app && \
    cp /build/target/release/dodopayments /app/dodopayments && \
    cp /build/.env /app/.env && \
    cd /app && \
    rm -rf /build

WORKDIR /app

CMD su - postgres -c "pg_ctl -D /var/lib/postgresql/data start" && \
    ./dodopayments
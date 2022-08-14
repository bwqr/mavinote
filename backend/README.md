# Backend

Backend is a RESTful web service which provides authorization and synchronization of notes across multiple applications.

### Prerequisites

To begin, you must have rust toolchain installed and accessible via **PATH** in your system, specifically **cargo** and **rustc**.
You can look at [rustup](https://rustup.rs/) to set up the rust programming language.

### Postgresql - Diesel

To create a persistent storage, **backend** uses **postgresql**. You can go
to [https://www.postgresql.org/download/](https://www.postgresql.org/download/) page and download appropriate package
for your system. After installing postgresql, you should create a database and a user.

To manage the database migrations, we need to install ```diesel_cli```. If you want to migrate database manually, you can
skip this part. However, before running the application, you must be sure that all queries inside
the ```migrations/**/up.sql``` files are executed. We can install ```diesel_cli``` by typing

```
cargo install diesel_cli --no-default-features --features postgres
```

In order to run migrations, type in this directory

```
diesel migration run --database-url=postgres://<username>:<password>@localhost/<database>
```

where **username**, **password** and **database** should be filled by you according to your postgresql setup.

## Configuration

Configurations reside in the `.env` file. You can use `.env.example` file as a base to create your own `.env` file.

You should configure **DATABASE_URL** inside the `api/.env` as we have done in diesel migration part.

A general overview of configurations

* **DATABASE_URL**: this environment variable specifies the database that backend connects. The format is in the form of
  ```postgres://<username>:<password>@<postgresql-socket-address>/<database>```
* **RUST_LOG**: specifies the log level of application. You can learn more about this variable from [here](https://docs.rs/env_logger/*/env_logger/index.html#enabling-logging).
* **BIND_ADDRESS**: the address that backend listens for tcp connections.
* **SECRET_KEY**: This is backend's secret key. It is used for cryptographic operations.

## Running

You can run backend by typing

```
cargo run
```

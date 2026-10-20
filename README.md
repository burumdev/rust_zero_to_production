# Zero to Production in Rust

This is an implementation of an Actix-web server newsletter app based on Luca Palmieri's book of the same name. It's a hands-on experience in professional Rust web stack.

The book version used as reference is 2020, so some of the libraries had subtle changes in their APIs, feature flags and types. For example instead of `once_cell` crate for the Lazy initialization in integration tests, I used `LazyLock` from `std::sync` module, as it is stabilized now.

## Running on the local machine
You'll need `psql` on your system and `sqlx-cli` on your Rust installation.
* run `./scripts/init_db.sh` # This will also migrate the database
* run `cargo run`

## Running containerized in Docker
* run `docker compose up`
* run `sqlx migrate run`


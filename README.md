# Zero to Production in Rust

This is an implementation of an Actix-web newsletter app based on Luca Palmieri's book of the same name. It's a hands-on experience in professional Rust web stack.

The book version used as reference is 2021. Some of the libraries had subtle changes in their APIs, feature flags and types in current versions. For example instead of `once_cell` crate for the Lazy initialization in integration tests, I used `LazyLock` from `std::sync`, as it is stabilized now.
Some of the libraries used in the book are used with original downgraded versions for compatibility and interoperability. As a result of it, this codebase might not have the latest security advisory patches applied and shouldn't be used or derived from in production (despite the name).

## Differences with the original project
* Added docker compose setup for easier ops and communication between app docker and database docker in local development environment. So it is easy to test the app in both development and production mode in local.
* DigitalOcean deployment setup is omitted
* SSL support is disabled
* Mailtrap API for email testing instead of Postmark. The free tier supports only text based emails.
* Email testing is done based on direct response from Mailtrap server instead of a mock server.
* The `modname/mod.rs` directory structure is not used. The more preferred approach `modname.rs` in root and submodules in `modname` directory is used.

# Running
Before running the app, make a copy of `.env-example` file as `.env` and fill in the credentials there
with your local credentials.
You'll need `psql` on your system and `sqlx-cli` on your Rust installation.

## Running on the local machine
* run `./scripts/init_db.sh`. This will also migrate the database
* run `cargo run`

## Running containerized in Docker
* run `docker compose up`
* run `sqlx migrate run`


![License](https://img.shields.io/badge/license-MIT-green.svg)
[![Cargo](https://img.shields.io/crates/v/diesel-connection.svg)](https://crates.io/crates/diesel-connection)
[![Documentation](https://docs.rs/diesel-connection/badge.svg)](https://docs.rs/diesel-connection)

Simple static diesel r2d2 connection pooling.

## ENV Configuration

* `DATABASE_URL` sets connection url

* `MAX_DB_CONNECTIONS` sets max connections within connection pool

The `dotenv` feature flag enables dotenv loading during pre-main static initialization via [dotenvy](https://crates.io/crates/dotenvy).

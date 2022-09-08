# diesel-connection

![License](https://img.shields.io/badge/license-MIT-green.svg)
[![Cargo](https://img.shields.io/crates/v/diesel-connection.svg)](https://crates.io/crates/diesel-connection)
[![Documentation](https://docs.rs/diesel-connection/badge.svg)](https://docs.rs/diesel-connection)

Static diesel r2d2 connection pooling.

Connection urls are provided by environment variables using [env-url](https://crates.io/crates/env-url) using the env variable `DATABASE_URL`.

`MAX_DB_CONNECTIONS` env sets max connections within connection pool

The `dotenv` feature flag enables automatic at-most-once dotenv loading via dotenvy. This is necessary because pool statics are initialized pre-main via [static_init](https://crates.io/crates/static_init).

```rust
use diesel_connection::{pg::get_connection, PoolError};

#[actix_rt::main]
async fn main() -> Result<(), PoolError> {
  let conn = get_connection()?;
}
```

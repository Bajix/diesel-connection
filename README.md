# diesel-connection

![License](https://img.shields.io/badge/license-MIT-green.svg)
[![Cargo](https://img.shields.io/crates/v/diesel-connection.svg)](https://crates.io/crates/diesel-connection)
[![Documentation](https://docs.rs/diesel-connection/badge.svg)](https://docs.rs/diesel-connection)

Static diesel r2d2 connection pooling.

`DATABASE_URL` env sets postgres database url within connection manager
`MAX_DB_CONNECTIONS` env sets max postgres connections within connection pool

The `tracing` feature flag substitutes connections instrumented with opentelemetry. See [diesel-tracing](https://crates.io/crates/diesel-tracing) for details.

```rust
#[actix_rt::main]
async fn main() -> Result<(), PoolError> {
  // DATABASE_URL can be set any time before the pool is lazily initialized on first use
  dotenv().expect("Unable to load .env file");
  env_logger::init();

  let conn = get_connection()?;
}
```

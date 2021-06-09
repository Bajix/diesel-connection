# diesel-connection

![License](https://img.shields.io/badge/license-MIT-green.svg)
[![Cargo](https://img.shields.io/crates/v/diesel-connection.svg)](https://crates.io/crates/diesel-connection)
[![Documentation](https://docs.rs/diesel-connection/badge.svg)](https://docs.rs/diesel-connection)

Static diesel r2d2 connection pooling.

`DATABASE_URL` env sets postgres database url within connection manager
`MAX_DB_CONNECTIONS` env sets max postgres connections within connection pool

This library uses [booter::boot()](https://docs.rs/booter/latest/booter/fn.boot.html) to initialize.

```rust
#[actix_rt::main]
async fn main() -> Result<(), PoolError> {
  // Env can be configured before booter::boot giving fine-grain initialization control
  dotenv().expect("Unable to load .env file");
  env_logger::init();
  // This calls registered initialization functions; with this we initialize our static connection pool
  booter::boot();
  let conn = get_connection()?;
}
```

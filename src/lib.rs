//! Static diesel r2d2 connection pooling.
//!
//! `DATABASE_URL` env sets postgres database url within connection manager
//!
//! `MAX_DB_CONNECTIONS` env sets max postgres connections within connection pool
//!
//! This library uses [booter::boot()](https://docs.rs/booter/latest/booter/fn.boot.html) to initialize.
//!
//! ```rust
//!   use diesel_connection::{get_connection, PoolError};
//!
//!   fn main() -> Result<(), PoolError> {
//!
//!   // Env can be configured before booter::boot giving fine-grain initialization control
//!   //dotenv().expect("Unable to load .env file");
//!
//!   // This calls registered initialization functions; i.e. to initialize our pool
//!   booter::boot();
//!
//!   let conn = get_connection()?;
//!   Ok(())
//! }
//! ```

#[cfg(feature = "mysql")]
pub type Backend = diesel::mysql::Mysql;

#[cfg(feature = "postgres")]
pub type Backend = diesel::pg::Pg;

#[cfg(feature = "sqlite")]
pub type Backend = diesel::sqlite::Sqlite;

#[cfg(all(feature = "tracing", feature = "mysql"))]
pub type Connection = diesel_tracing::mysql::InstrumentedMysqlConnection;

#[cfg(all(not(feature = "tracing"), feature = "mysql"))]
pub type Connection = diesel::mysql::MysqlConnection;

#[cfg(all(feature = "tracing", feature = "postgres"))]
pub type Connection = diesel_tracing::pg::InstrumentedPgConnection;

#[cfg(all(not(feature = "tracing"), feature = "postgres"))]
pub type Connection = diesel::pg::PgConnection;

#[cfg(all(feature = "tracing", feature = "sqlite"))]
pub type Connection = diesel_tracing::sqlite::InstrumentedSqliteConnection;

#[cfg(all(not(feature = "tracing"), feature = "sqlite"))]
pub type Connection = diesel::sqlite::SqliteConnection;

pub use diesel::r2d2::PoolError;
use diesel::r2d2::{self, ConnectionManager};
use once_cell::sync::OnceCell;

pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<Connection>>;

static POOL: OnceCell<Pool> = OnceCell::new();

booter::call_on_boot!({
  let database_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL env variable");

  let max_connections = std::env::var("MAX_DB_CONNECTIONS")
    .unwrap_or_else(|_| String::from("20"))
    .parse::<u32>()
    .unwrap();
  let connection_manager = ConnectionManager::<Connection>::new(database_url);

  let connection_pool = r2d2::Pool::builder()
    .max_size(max_connections)
    .build(connection_manager)
    .expect("Timeout establishing initial postgres connection");

  if POOL.set(connection_pool).is_err() {
    panic!("connection pool already initialized");
  }
});

/// Retrieves a connection from the global connection pool.
pub fn get_connection() -> Result<PooledConnection, PoolError> {
  booter::assert_booted();
  POOL.get().unwrap().get()
}

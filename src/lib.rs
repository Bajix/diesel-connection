//! Static diesel r2d2 connection pooling.
//!
//! `DATABASE_URL` env sets postgres database url within connection manager
//!
//! `MAX_DB_CONNECTIONS` env sets max postgres connections within connection pool
//!
//! ```rust
//!   use diesel_connection::{get_connection, PoolError};
//!
//!   fn main() -> Result<(), PoolError> {
//!     // DATABASE_URL can be set any time before the pool is lazily initialized on first use
//!     dotenv().expect("Unable to load .env file");
//!     env_logger::init();
//!
//!     let conn = get_connection()?;
//!   }
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
use once_cell::sync::Lazy;

pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<Connection>>;

static POOL: Lazy<Pool> = Lazy::new(|| {
  let database_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL env variable");

  let max_connections = std::env::var("MAX_DB_CONNECTIONS")
    .unwrap_or_else(|_| String::from("20"))
    .parse::<u32>()
    .unwrap();

  let connection_manager = ConnectionManager::<Connection>::new(database_url);

  r2d2::Pool::builder()
    .max_size(max_connections)
    .build(connection_manager)
    .expect("Cannot establish connection pool: invalid DATABASE_URL")
});

/// Retrieves a connection from a lazily initialized connection pool
pub fn get_connection() -> Result<PooledConnection, PoolError> {
  (&*POOL).get()
}

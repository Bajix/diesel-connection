//! Static diesel r2d2 connection pooling with env configuration
#![feature(doc_cfg)]
#![allow(rustdoc::private_intra_doc_links)]
extern crate self as diesel_connection;
#[doc(hidden)]
pub extern crate static_init;
pub use derive_diesel_connection::StaticConnectionPool;
use diesel::{
  r2d2::{self, ConnectionManager, PoolError},
  Connection,
};
use env_url::*;
pub trait ConnectionInfo: ServiceURL {
  type Connection: Connection + 'static;

  fn create_pool() -> Result<r2d2::Pool<ConnectionManager<Self::Connection>>, ParseError> {
    #[cfg(feature = "dotenv")]
    dotenvy::dotenv().ok();

    let database_url = <Self as ServiceURL>::service_url()?;

    let max_connections = std::env::var("MAX_DB_CONNECTIONS")
      .unwrap_or_else(|_| String::from("20"))
      .parse::<u32>()
      .unwrap();

    let connection_manager: ConnectionManager<Self::Connection> =
      ConnectionManager::new(database_url);
    let pool = r2d2::Pool::builder()
      .max_size(max_connections)
      .build_unchecked(connection_manager);
    Ok(pool)
  }
}

/// Trait for defining static connection pool
pub trait StaticPoolContext: ConnectionInfo {
  fn pool() -> &'static r2d2::Pool<ConnectionManager<<Self>::Connection>>;

  fn get_connection(
  ) -> Result<r2d2::PooledConnection<ConnectionManager<Self::Connection>>, PoolError> {
    <Self as StaticPoolContext>::pool().get()
  }
}

/// Types for MySQL connections. Enable via `mysql` feature flag
#[cfg(any(feature = "mysql", doc))]
pub mod mysql {
  use super::*;

  #[cfg(feature = "tracing")]
  pub type Connection = diesel_tracing::mysql::InstrumentedMysqlConnection;

  #[cfg(not(feature = "tracing"))]
  pub type Connection = diesel::mysql::MysqlConnection;

  pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<Connection>>;

  #[derive(EnvURL, StaticConnectionPool)]
  #[env_url(env_prefix = "MYSQL", default = "mysql://localhost:3306")]
  pub struct ConnectionPool;

  impl ConnectionInfo for ConnectionPool {
    type Connection = Connection;
  }

  /// Get mysql connection from pool. Use `MYSQL_URL` env variable to set connection url
  pub fn get_connection() -> Result<PooledConnection, PoolError> {
    <ConnectionPool as StaticPoolContext>::get_connection()
  }
}

/// Types for Postgres connections. Enable via `postgres` feature flag
#[cfg(any(feature = "postgres", doc))]
pub mod pg {
  use super::*;

  #[cfg(feature = "tracing")]
  pub type Connection = diesel_tracing::pg::InstrumentedPgConnection;

  #[cfg(not(feature = "tracing"))]
  pub type Connection = diesel::pg::PgConnection;

  pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<Connection>>;

  #[derive(EnvURL, StaticConnectionPool)]
  #[env_url(env_prefix = "PG", default = "postgresql://localhost:5432")]
  pub struct ConnectionPool;

  impl ConnectionInfo for ConnectionPool {
    type Connection = Connection;
  }

  /// Get mysql connection from pool. Use `PG_URL` env variable to set connection url
  pub fn get_connection() -> Result<PooledConnection, PoolError> {
    <ConnectionPool as StaticPoolContext>::get_connection()
  }
}

/// Types for SQLite connections. Enable via `sqlite` feature flag
#[cfg(any(feature = "sqlite", doc))]
pub mod sqlite {
  use super::*;

  #[cfg(feature = "tracing")]
  pub type Connection = diesel_tracing::sqlite::InstrumentedSqliteConnection;

  #[cfg(not(feature = "tracing"))]
  pub type Connection = diesel::sqlite::SqliteConnection;

  pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<Connection>>;

  #[derive(EnvURL, StaticConnectionPool)]
  #[env_url(env_prefix = "SQLITE", default = "sqlite://./db.sqlite")]
  pub struct ConnectionPool;

  impl ConnectionInfo for ConnectionPool {
    type Connection = Connection;
  }

  /// Get sqlite connection from pool. Use `SQLITE_URL` env variable to set connection url
  pub fn get_connection() -> Result<PooledConnection, PoolError> {
    <ConnectionPool as StaticPoolContext>::get_connection()
  }
}

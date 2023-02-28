//! Static diesel r2d2 connection pooling with env configuration
//!
//! Composible connection urls are provided by environment variables using [env-url](https://crates.io/crates/env-url) using the `DATABASE` prefix:
//!
//! ```
//!  | ENV                     |                            |
//!  | -----------------------:|:--------------------------:|
//!  | DATABASE_URL            | set connection url         |
//!  | DATABASE_HOST           | set url host               |
//!  | DATABASE_PORT           | set url port               |
//!  | DATABASE_PATH           | set selected database      |
//!  | DATABASE_USERINFO       | set connnection userinfo   |
//! ```
//!
//! `MAX_DB_CONNECTIONS` env sets max connections within connection pool
//!
//! The `dotenv` feature flag enables automatic at-most-once dotenv loading via dotenvy. This is necessary because pool statics are initialized pre-main via [static_init](https://crates.io/crates/static_init).
//!
//! ```rust
//! use diesel_connection::{pg::get_connection, PoolError};
//!
//! #[actix_rt::main]
//! async fn main() -> Result<(), PoolError> {
//!   let conn = get_connection()?;
//! }
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(rustdoc::private_intra_doc_links)]
extern crate self as diesel_connection;
#[doc(hidden)]
pub extern crate static_init;
use cfg_block::cfg_block;
pub use derive_diesel_connection::StaticConnectionPool;
use diesel::r2d2::{self, ConnectionManager, PoolError, R2D2Connection};
use env_url::*;
pub trait ConnectionInfo: ServiceURL {
  type Connection: R2D2Connection + 'static;

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

cfg_block! {
  #[cfg(any(doc, all(feature = "postgres", not(feature = "mysql"), not(feature = "sqlite"))))] {
    #[derive(EnvURL, StaticConnectionPool)]
    #[env_url(env_prefix = "DATABASE", default = "postgresql://localhost:5432")]
    /// Static connection pool type
    pub struct ConnectionPool;

    impl ConnectionInfo for ConnectionPool {
      type Connection = pg::Connection;
    }

    /// Get postgress connection from pool. Use `DATABASE_URL` env variable to set connection url
    pub fn get_connection() -> Result<pg::PooledConnection, PoolError> {
      <ConnectionPool as StaticPoolContext>::get_connection()
    }
  }

  #[cfg(all(feature = "mysql", not(feature = "postgres"), not(feature = "sqlite")))] {
    #[derive(EnvURL, StaticConnectionPool)]
    #[env_url(env_prefix = "DATABASE", default = "mysql://localhost:3306")]
    /// Static connection pool type
    pub struct ConnectionPool;

    impl ConnectionInfo for ConnectionPool {
      type Connection = mysql::Connection;
    }

    /// Get mysql connection from pool. Use `DATABASE_URL` env variable to set connection url
    pub fn get_connection() -> Result<mysql::PooledConnection, PoolError> {
      <ConnectionPool as StaticPoolContext>::get_connection()
    }
  }

  #[cfg(all(feature = "sqlite", not(feature = "mysql"), not(feature = "postgres")))] {
    #[derive(EnvURL, StaticConnectionPool)]
    #[env_url(env_prefix = "DATABASE", default = "sqlite://./db.sqlite")]
    /// Static connection pool type
    pub struct ConnectionPool;

    impl ConnectionInfo for ConnectionPool {
      type Connection = sqlite::Connection;
    }

    /// Get sqlite connection from pool. Use `DATABASE_URL` env variable to set connection url
    pub fn get_connection() -> Result<sqlite::PooledConnection, PoolError> {
      <ConnectionPool as StaticPoolContext>::get_connection()
    }
  }
}

/// Types for MySQL connections.
#[cfg(any(doc, feature = "mysql"))]
#[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
pub mod mysql {
  use super::*;

  pub type Connection = diesel::mysql::MysqlConnection;
  pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<Connection>>;
}

/// Types for Postgres connections.
#[cfg(any(doc, feature = "postgres"))]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres")))]
pub mod pg {
  use super::*;

  pub type Connection = diesel::pg::PgConnection;
  pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<Connection>>;
}

/// Types for SQLite connections.
#[cfg(any(doc, feature = "sqlite"))]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
pub mod sqlite {
  use super::*;

  pub type Connection = diesel::sqlite::SqliteConnection;
  pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<Connection>>;
}

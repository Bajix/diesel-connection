//! Static diesel r2d2 connection pooling.
//!
//! `DATABASE_URL` env sets postgres database url within connection manager
//!
//! `MAX_DB_CONNECTIONS` env sets max postgres connections within connection pool
//!
//! ```rust
//!
//! #[actix_rt::main]
//! async fn main() -> Result<(), PoolError> {
//!   // Env can be configured before booter::boot giving fine-grain initialization control
//!   dotenv().expect("Unable to load .env file");
//!   env_logger::init();
//!   // This calls registered initialization functions; i.e. to initialize our pool
//!   booter::boot();
//!   let conn = get_connection()?;
//! }
//! ```

use diesel::pg::PgConnection;
pub use diesel::r2d2::PoolError;
use diesel::r2d2::{self, ConnectionManager};
use once_cell::sync::OnceCell;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

static POOL: OnceCell<Pool> = OnceCell::new();

booter::call_on_boot!({
  let database_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL env variable");
  let max_connections = std::env::var("MAX_DB_CONNECTIONS")
    .unwrap_or_else(|_| String::from("20"))
    .parse::<u32>()
    .unwrap();
  let connection_manager = ConnectionManager::<PgConnection>::new(database_url);

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

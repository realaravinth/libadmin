//! meta operations like migration and connecting to a database
use crate::dev::*;

/// Database operations trait(migrations, pool creation and fetching connection from pool)
pub trait DBOps: GetConnection + Migrate + Connect {}

/// Get database connection
#[async_trait]
pub trait GetConnection {
    /// database connection type
    type Conn;
    /// database specific error-type
    type Error: std::error::Error;
    /// get connection from connection pool
    async fn get_conn(&self) -> DBResult<Self::Conn, Self::Error>;
}

/// Create databse connection
#[async_trait]
pub trait Connect {
    /// database connection type
    type Config;
    /// database specific pool-type
    type Pool: GetConnection;
    /// database specific error-type
    type Error: std::error::Error;
    /// create connection pool
    async fn connect(config: Self::Config) -> DBResult<Self::Pool, Self::Error>;
}

/// database migrations
#[async_trait]
pub trait Migrate: GetConnection {
    /// database specific error-type
    type Error: std::error::Error;
    /// run migrations
    async fn migrate(&self) -> DBResult<(), <Self as Migrate>::Error>;
}

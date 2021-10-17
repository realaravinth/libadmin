use async_trait::async_trait;

pub mod account;
pub mod auth;
pub mod errors;

use errors::*;

pub trait Database: DBOps + auth::Auth + account::Account {}

pub trait DBOps: GetConnection + Migrate + Connect {}

/// Get database connection
#[async_trait]
pub trait GetConnection {
    /// Database connection type
    type Conn;
    type Error: std::error::Error;
    async fn get_conn(&self) -> DBResult<Self::Conn, Self::Error>;
}

/// Create databse connection
#[async_trait]
pub trait Connect {
    /// Database connection type
    type Config;
    type Pool;
    type Error: std::error::Error;
    async fn connect(config: Self::Config) -> DBResult<Self::Pool, Self::Error>;
}

/// database migrations
#[async_trait]
pub trait Migrate {
    type Error: std::error::Error;
    async fn migrate<C: GetConnection>(&self) -> DBResult<(), Self::Error>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

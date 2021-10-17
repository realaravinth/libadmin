use async_trait::async_trait;

pub mod auth;
pub mod errors;

use errors::*;

pub trait Database: DBConn + Migrate + auth::Auth {}

/// Get database connection
pub trait DBConn {
    /// Database connection type
    type Conn;
    type Error: std::error::Error;
    fn conn(&self) -> DBResult<Self::Conn, Self::Error>;
}

/// database migrations
#[async_trait]
pub trait Migrate {
    type Error: std::error::Error;
    async fn migrate<C: DBConn>(c: C) -> DBResult<(), Self::Error>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

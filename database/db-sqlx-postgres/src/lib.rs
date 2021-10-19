#![deny(missing_docs)]
//! # `libadmin` database operations implemented using sqlx postgres
//!
//! [`LibAdminDatabase`](LibAdminDatabase) is implemented on [Database].
use db_core::dev::*;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub mod account;
pub mod auth;
mod errors;
#[cfg(test)]
pub mod tests;

/// Database pool. All database functionallity(`libadmin` traits) are implemented on this
/// data structure
pub struct Database {
    /// database pool
    pub pool: PgPool,
}

/// Configure database pool
pub struct ConnectionOptions {
    /// Pool options
    pub pool_options: PgPoolOptions,
    /// database URL
    pub url: String,
}

impl LibAdminDatabase for Database {}

pub mod dev {
    //! useful imports for supporting a new database
    pub use super::errors::*;
    pub use super::Database;
    pub use db_core::dev::*;
    pub use prelude::*;
    pub use sqlx::Error;
}

pub mod prelude {
    //! useful imports for users working with a supported database
    pub use super::*;
    pub use db_core::prelude::*;
}

impl DBOps for Database {}

#[async_trait]
impl Connect for ConnectionOptions {
    type Error = sqlx::Error;
    type Pool = Database;
    /// create connection pool
    async fn connect(self) -> DBResult<Self::Pool, Self::Error> {
        let pool = self
            .pool_options
            .connect(&self.url)
            .await
            .map_err(DBError::DBError)?;
        Ok(Database { pool })
    }
}

#[async_trait]
impl GetConnection for Database {
    type Conn = PgPool;
    type Error = sqlx::Error;
    /// get connection from connection pool
    async fn get_conn(&self) -> DBResult<Self::Conn, Self::Error> {
        Ok(self.pool.clone())
    }
}

#[async_trait]
impl Migrate for Database {
    type Error = sqlx::migrate::MigrateError;
    async fn migrate(&self) -> DBResult<(), <Self as Migrate>::Error> {
        sqlx::migrate!("./migrations/")
            .run(&self.pool)
            .await
            .map_err(DBError::DBError)?;
        Ok(())
    }
}

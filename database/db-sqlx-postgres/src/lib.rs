use db_core::dev::*;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub mod account;
pub mod auth;
pub mod errors;

pub struct Database {
    pub pool: PgPool,
}

pub struct ConnectionOptions {
    pub pool_options: PgPoolOptions,
    pub url: String,
}

impl LibAdminDatabase for Database {}

pub mod dev {
    pub use super::errors::*;
    pub use super::Database;
    pub use db_core::dev::*;
    pub use prelude::*;
    pub use sqlx::Error;
}

pub mod prelude {
    pub use super::*;
    pub use db_core::prelude::*;
}

impl DBOps for Database {}
#[async_trait]
impl Connect for Database {
    type Error = sqlx::Error;
    type Pool = Database;
    type Config = ConnectionOptions;

    async fn connect(config: Self::Config) -> DBResult<Self::Pool, Self::Error> {
        let pool = config
            .pool_options
            .connect(&config.url)
            .await
            .map_err(DBError::DBError)?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl GetConnection for Database {
    type Conn = PgPool;
    type Error = sqlx::Error;
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

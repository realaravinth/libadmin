use async_trait::async_trait;

use db_core::errors::*;
use db_core::*;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub struct Database {
    pub pool: PgPool,
}

pub struct ConnectionOptions {
    pub pool_options: PgPoolOptions,
    pub url: String,
}

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
            .map_err(|e| DBError::DBError(e))?;
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
    async fn migrate<C: GetConnection>(&self) -> DBResult<(), Self::Error> {
        sqlx::migrate!("./migrations/")
            .run(&self.pool)
            .await
            .map_err(|e| DBError::DBError(e))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

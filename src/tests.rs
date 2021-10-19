use crate::*;
use std::sync::Arc;

use std::env;

pub mod sqlx_postgres {
    use super::*;
    use db_sqlx_postgres::ConnectionOptions;
    use sqlx::postgres::PgPoolOptions;

    pub async fn get_data() -> Arc<Data> {
        let url = env::var("POSTGRES_DATABASE_URL").unwrap();
        let mut settings = Settings::default();
        settings.database.url = url.clone();
        let pool_options = PgPoolOptions::new().max_connections(2);
        let connection_options = ConnectionOptions { pool_options, url };

        return Data::new(connection_options, settings).await;
    }
}

pub mod sqlx_sqlite {
    use super::*;
    use db_sqlx_sqlite::ConnectionOptions;
    use sqlx::sqlite::SqlitePoolOptions;

    pub async fn get_data() -> Arc<Data> {
        let url = env::var("SQLITE_DATABASE_URL").unwrap();
        let mut settings = Settings::default();
        settings.database.url = url.clone();

        let pool_options = SqlitePoolOptions::new().max_connections(2);
        let connection_options = ConnectionOptions { pool_options, url };

        return Data::new(connection_options, settings).await;
    }
}

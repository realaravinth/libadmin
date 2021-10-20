use crate::*;
use std::sync::Arc;

use std::env;

pub mod sqlx_postgres {
    use super::*;
    use db_sqlx_postgres::{ConnectionOptions, Fresh};
    use sqlx::postgres::PgPoolOptions;

    pub async fn get_data() -> (Box<dyn LibAdminDatabase>, Arc<Data>) {
        let url = env::var("POSTGRES_DATABASE_URL").unwrap();
        let mut settings = Settings::default();
        settings.database.url = url.clone();
        let pool_options = PgPoolOptions::new().max_connections(2);
        let connection_options = ConnectionOptions::Fresh(Fresh { pool_options, url });

        let db = Box::new(connection_options.connect().await.unwrap());

        (db, Data::new(settings).await)
    }
}

pub mod sqlx_sqlite {
    use super::*;
    use db_sqlx_sqlite::{ConnectionOptions, Fresh};
    use sqlx::sqlite::SqlitePoolOptions;

    pub async fn get_data() -> (Box<dyn LibAdminDatabase>, Arc<Data>) {
        let url = env::var("SQLITE_DATABASE_URL").unwrap();
        let mut settings = Settings::default();
        settings.database.url = url.clone();

        let pool_options = SqlitePoolOptions::new().max_connections(2);
        let connection_options = ConnectionOptions::Fresh(Fresh { pool_options, url });

        let db = Box::new(connection_options.connect().await.unwrap());
        (db, Data::new(settings).await)
    }
}

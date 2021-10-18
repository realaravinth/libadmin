use crate::*;
use std::env;

use db_sqlx_postgres::ConnectionOptions;
use once_cell::sync::OnceCell;
use sqlx::postgres::PgPoolOptions;

/// Settings
pub static DATA: OnceCell<Arc<Data<db_sqlx_postgres::Database>>> = OnceCell::new();

pub async fn get_data() -> Arc<Data<db_sqlx_postgres::Database>> {
    if DATA.get().is_none() {
        init(Settings::default());
        let url = env::var("POSTGRES_DATABASE_URL").unwrap();
        let pool_options = PgPoolOptions::new().max_connections(4);
        let connection_options = ConnectionOptions { pool_options, url };

        let data = Data::new(connection_options).await;
        DATA.get_or_init(|| data);
    }
    DATA.get().unwrap().clone()
}

//#[actix_rt::test]
//async fn everyting_works() {
//    const EMAIL: &str = "libaduser@foo.com";
//    const EMAIL2: &str = "libadminuser2@foo.com";
//    const NAME: &str = "libaduser";
//    const NAME2: &str = "libaduser2";
//    const NAME3: &str = "libaduser3";
//    const NAME4: &str = "libaduser4";
//    const NAME5: &str = "libaduser5";
//    const NAME6: &str = "libaduser6";
//    const NAME7: &str = "libaduser7";
//    const PASSWORD: &str = "pasdfasdfasdfadf";
//    const SECRET1: &str = "libadinsecret1";
//    const SECRET2: &str = "libadinsecret2";
//    const SECRET3: &str = "libadinsecret3";
//    const SECRET4: &str = "libadinsecret4";
//
//    let url = env::var("POSTGRES_DATABASE_URL").unwrap();
//    let pool_options = PgPoolOptions::new().max_connections(2);
//    let connection_options = ConnectionOptions { pool_options, url };
//    let data = Data::new(connection_options).await;
//    let register_payload = data.register();
//    //    let db = connection_options.connect().await.unwrap();
//    //
//    //    db.migrate().await.unwrap();
//    //    email_register_works(&db, EMAIL, NAME, PASSWORD, SECRET1, NAME5).await;
//    //    username_register_works(&db, NAME2, PASSWORD, SECRET2).await;
//    //    duplicate_secret_guard_works(&db, NAME3, PASSWORD, NAME4, SECRET3, SECRET2).await;
//    //    duplicate_username_and_email(&db, NAME6, NAME7, EMAIL2, PASSWORD, SECRET4, NAME, EMAIL).await;
//    //    let creds = Creds {
//    //        username: NAME.into(),
//    //        password: SECRET4.into(),
//    //    };
//    //    db.update_password(&creds).await.unwrap();
//}

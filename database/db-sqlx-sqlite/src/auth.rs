//! Authentication and registration operations
use crate::dev::*;

impl Auth for Database {}

use crate::Database;

pub mod login {
    use super::*;

    impl Login for Database {}
    #[async_trait]
    impl EmailLogin for Database {
        type Error = Error;
        async fn email_login(&self, email: &str) -> DBResult<Creds, <Self as EmailLogin>::Error> {
            sqlx::query_as!(
                Creds,
                r#"SELECT username, password  FROM admin_users WHERE email = ($1)"#,
                email,
            )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                Error::RowNotFound => DBError::AccountNotFound,
                e => DBError::DBError(e),
            })
        }
    }

    #[async_trait]
    impl UsernameLogin for Database {
        type Error = Error;
        async fn username_login(
            &self,
            username: &str,
        ) -> DBResult<Password, <Self as UsernameLogin>::Error> {
            sqlx::query_as!(
                Password,
                r#"SELECT password  FROM admin_users WHERE username = ($1)"#,
                username,
            )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                Error::RowNotFound => DBError::AccountNotFound,
                e => DBError::DBError(e),
            })
        }
    }
}

pub mod register {

    use super::*;

    impl Register for Database {}

    #[async_trait]
    impl EmailRegister for Database {
        type Error = Error;
        async fn email_register(
            &self,
            payload: &EmailRegisterPayload,
        ) -> DBResult<(), <Self as EmailRegister>::Error> {
            sqlx::query!(
                "insert into admin_users 
        (username , password, email, secret) values ($1, $2, $3, $4)",
                payload.username,
                payload.password,
                payload.email,
                payload.secret,
            )
            .execute(&self.pool)
            .await
            .map_err(map_register_err)?;
            Ok(())
        }
    }

    #[async_trait]
    impl UsernameRegister for Database {
        type Error = Error;
        async fn username_register(
            &self,
            payload: &UsernameRegisterPayload,
        ) -> DBResult<(), <Self as UsernameRegister>::Error> {
            sqlx::query!(
                "INSERT INTO admin_users 
        (username , password,  secret) VALUES ($1, $2, $3)",
                payload.username,
                payload.password,
                payload.secret,
            )
            .execute(&self.pool)
            .await
            .map_err(map_register_err)?;
            Ok(())
        }
    }
}

//! Authentication and registration operations
use sqlx::Error;
use std::borrow::Cow;

use db_core::dev::*;

use crate::Database;

pub mod login {
    use super::*;

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
                &payload.username,
                &payload.password,
                &payload.email,
                &payload.secret,
            )
            .execute(&self.pool)
            .await
            .map_err(|e| map_register_err(e))?;
            Ok(())
        }
    }

    fn map_register_err(e: Error) -> DBError<Error> {
        if let Error::Database(err) = e {
            if err.code() == Some(Cow::from("23505")) {
                let msg = err.message();
                if msg.contains("admin_users_username_key") {
                    DBError::DuplicateUsername
                } else if msg.contains("admin_users_email_key") {
                    DBError::DuplicateEmail
                } else if msg.contains("admin_users_secret_key") {
                    DBError::DuplicateSecret
                } else {
                    DBError::DBError(Error::Database(err).into())
                }
            } else {
                DBError::DBError(Error::Database(err).into())
            }
        } else {
            DBError::DBError(e)
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
                &payload.username,
                &payload.password,
                &payload.secret,
            )
            .execute(&self.pool)
            .await
            .map_err(|e| map_register_err(e))?;
            Ok(())
        }
    }
}

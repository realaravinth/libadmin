use db_core::dev::*;

use sqlx::sqlite::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

pub mod errors;
#[cfg(test)]
pub mod tests;

pub struct Database {
    pub pool: SqlitePool,
}

pub struct ConnectionOptions {
    pub pool_options: SqlitePoolOptions,
    pub url: String,
}

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

#[async_trait]
impl Connect for ConnectionOptions {
    type Pool = Database;
    async fn connect(self) -> DBResult<Self::Pool> {
        let pool = self
            .pool_options
            .connect(&self.url)
            .await
            .map_err(|e| DBError::DBError(format!("{:?}", e)))?;
        Ok(Database { pool })
    }
}

use dev::*;

#[async_trait]
impl Migrate for Database {
    async fn migrate(&self) -> DBResult<()> {
        sqlx::migrate!("./migrations/")
            .run(&self.pool)
            .await
            .map_err(|e| DBError::DBError(format!("{:?}", e)))?;
        Ok(())
    }
}

#[async_trait]
impl LibAdminDatabase for Database {
    async fn email_login(&self, email: &str) -> DBResult<Creds> {
        sqlx::query_as!(
            Creds,
            r#"SELECT username, password  FROM admin_users WHERE email = ($1)"#,
            email,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            Error::RowNotFound => DBError::AccountNotFound,
            e => DBError::DBError(format!("{:?}", e)),
        })
    }

    async fn username_login(&self, username: &str) -> DBResult<Password> {
        sqlx::query_as!(
            Password,
            r#"SELECT password  FROM admin_users WHERE username = ($1)"#,
            username,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            Error::RowNotFound => DBError::AccountNotFound,
            e => DBError::DBError(format!("{:?}", e)),
        })
    }

    async fn email_register(&self, payload: &EmailRegisterPayload) -> DBResult<()> {
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

    async fn username_register(&self, payload: &UsernameRegisterPayload) -> DBResult<()> {
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

    async fn update_email(&self, payload: &UpdateEmailPayload) -> DBResult<()> {
        sqlx::query!(
            "UPDATE admin_users set email = $1
        WHERE username = $2",
            payload.email,
            payload.username,
        )
        .execute(&self.pool)
        .await
        .map_err(map_register_err)?;
        Ok(())
    }
    async fn update_password(&self, payload: &Creds) -> DBResult<()> {
        sqlx::query!(
            "UPDATE admin_users set password = $1
        WHERE username = $2",
            payload.password,
            payload.username,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DBError::DBError(format!("{:?}", e)))?;
        Ok(())
    }

    async fn email_exists(&self, email: &str) -> DBResult<bool> {
        let exists;
        match sqlx::query!("SELECT id from admin_users WHERE email = $1", email)
            .fetch_one(&self.pool)
            .await
        {
            Ok(_) => exists = true,
            Err(Error::RowNotFound) => exists = false,
            Err(e) => return Err(DBError::DBError(format!("{:?}", e))),
        };

        Ok(exists)
    }

    async fn delete_account(&self, username: &str) -> DBResult<()> {
        sqlx::query!("DELETE FROM admin_users WHERE username = ($1)", username,)
            .execute(&self.pool)
            .await
            .map_err(|e| DBError::DBError(format!("{:?}", e)))?;
        Ok(())
    }

    async fn username_exists(&self, username: &str) -> DBResult<bool> {
        let exists;
        match sqlx::query!("SELECT id from admin_users WHERE username = $1", username)
            .fetch_one(&self.pool)
            .await
        {
            Ok(_) => exists = true,
            Err(Error::RowNotFound) => exists = false,
            Err(e) => return Err(DBError::DBError(format!("{:?}", e))),
        };

        Ok(exists)
    }

    async fn update_username(&self, payload: &UpdateUsernamePayload) -> DBResult<()> {
        sqlx::query!(
            "UPDATE admin_users set username = $1 WHERE username = $2",
            payload.new_username,
            payload.old_username,
        )
        .execute(&self.pool)
        .await
        .map_err(map_register_err)?;
        Ok(())
    }
    async fn update_secret(&self, username: &str, secret: &str) -> DBResult<()> {
        sqlx::query!(
            "UPDATE admin_users set secret = $1
        WHERE username = $2",
            secret,
            username,
        )
        .execute(&self.pool)
        .await
        .map_err(map_register_err)?;
        Ok(())
    }

    async fn get_secret(&self, username: &str) -> DBResult<String> {
        struct Secret {
            secret: String,
        }
        let secret = sqlx::query_as!(
            Secret,
            r#"SELECT secret  FROM admin_users WHERE username = ($1)"#,
            username,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            Error::RowNotFound => DBError::AccountNotFound,
            e => DBError::DBError(format!("{:?}", e)),
        })?;

        Ok(secret.secret)
    }
}

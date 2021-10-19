use crate::dev::*;

impl Account for Database {}

#[async_trait]
impl UpdateEmail for Database {
    type Error = Error;
    async fn update_email(
        &self,
        payload: &UpdateEmailPayload,
    ) -> DBResult<(), <Self as UpdateEmail>::Error> {
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
}

/// Update password of specified user in database
#[async_trait]
impl UpdatePassword for Database {
    /// Database specific error-type
    type Error = Error;
    /// Update password of specified user in database
    async fn update_password(
        &self,
        payload: &Creds,
    ) -> DBResult<(), <Self as UpdatePassword>::Error> {
        sqlx::query!(
            "UPDATE admin_users set password = $1
        WHERE username = $2",
            payload.password,
            payload.username,
        )
        .execute(&self.pool)
        .await
        .map_err(DBError::DBError)?;
        Ok(())
    }
}

/// Check if an email exists in the database
#[async_trait]
impl EmailExists for Database {
    /// Database specific error-type
    type Error = Error;
    /// check if an email exists in the database
    async fn email_exists(&self, email: &str) -> DBResult<bool, <Self as EmailExists>::Error> {
        let exists;
        match sqlx::query!("SELECT id from admin_users WHERE email = $1", email)
            .fetch_one(&self.pool)
            .await
        {
            Ok(_) => exists = true,
            Err(Error::RowNotFound) => exists = false,
            Err(e) => return Err(DBError::DBError(e)),
        };

        Ok(exists)
    }
}

/// Delete an account
#[async_trait]
impl DeleteAccount for Database {
    /// Database specific error-type
    type Error = Error;
    /// delete account from database
    async fn delete_account(&self, username: &str) -> DBResult<(), <Self as DeleteAccount>::Error> {
        sqlx::query!("DELETE FROM admin_users WHERE username = ($1)", username,)
            .execute(&self.pool)
            .await
            .map_err(DBError::DBError)?;
        Ok(())
    }
}

/// Check if a username exists on the database
#[async_trait]
impl UsernameExists for Database {
    /// Database specific error-type
    type Error = Error;
    /// check if a username exists in the database
    async fn username_exists(
        &self,
        username: &str,
    ) -> DBResult<bool, <Self as UsernameExists>::Error> {
        let exists;
        match sqlx::query!("SELECT id from admin_users WHERE username = $1", username)
            .fetch_one(&self.pool)
            .await
        {
            Ok(_) => exists = true,
            Err(Error::RowNotFound) => exists = false,
            Err(e) => return Err(DBError::DBError(e)),
        };

        Ok(exists)
    }
}

/// update username in database
#[async_trait]
impl UpdateUsername for Database {
    /// Database specific error-type
    type Error = Error;
    /// update username in database
    async fn update_username(
        &self,
        payload: &UpdateUsernamePayload,
    ) -> DBResult<(), <Self as UpdateUsername>::Error> {
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
}

/// update secret in database
#[async_trait]
impl UpdateSecret for Database {
    /// Database specific error-type
    type Error = Error;
    /// update username in database
    async fn update_secret(
        &self,
        username: &str,
        secret: &str,
    ) -> DBResult<(), <Self as UpdateSecret>::Error> {
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
}

/// get secret from database
#[async_trait]
impl GetSecret for Database {
    /// Database specific error-type
    type Error = Error;
    /// update username in database
    async fn get_secret(&self, username: &str) -> DBResult<String, <Self as UpdateSecret>::Error> {
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
            e => DBError::DBError(e),
        })?;

        Ok(secret.secret)
    }
}

//! Account operations: update email, password and account secrets, etc.
use crate::dev::*;

/// Top-level trait grouping all account management operations
pub trait Account:
    UpdateEmail
    + EmailExists
    + DeleteAccount
    + UpdatePassword
    + UpdateUsername
    + UsernameExists
    + UpdateSecret
{
}

pub use crate::auth::login::Creds;

/// payload to update email in the database
#[derive(Clone, Debug)]
pub struct UpdateEmailPayload<'a> {
    /// name of the user who's email is to be updated
    pub name: &'a str,
    /// new email
    pub email: &'a str,
}

/// Update email of specified user in database
#[async_trait]
pub trait UpdateEmail: GetConnection {
    /// Database specific error-type
    type Error: std::error::Error;
    /// Update email of specified user in database
    async fn update_email(
        &self,
        payload: &UpdateEmailPayload,
    ) -> DBResult<(), <Self as UpdateEmail>::Error>;
}

/// Update password of specified user in database
#[async_trait]
pub trait UpdatePassword: GetConnection {
    /// Database specific error-type
    type Error: std::error::Error;
    /// Update password of specified user in database
    async fn update_password(
        &self,
        payload: &Creds,
    ) -> DBResult<(), <Self as UpdatePassword>::Error>;
}

/// Check if an email exists in the database
#[async_trait]
pub trait EmailExists: GetConnection {
    /// Database specific error-type
    type Error: std::error::Error;
    /// check if an email exists in the database
    async fn email_exists(&self, email: &str) -> DBResult<bool, <Self as EmailExists>::Error>;
}

/// Delete an account
#[async_trait]
pub trait DeleteAccount: GetConnection {
    /// Database specific error-type
    type Error: std::error::Error;
    /// delete account from database
    async fn delete_account(&self, username: &str) -> DBResult<(), <Self as DeleteAccount>::Error>;
}

/// Check if a username exists on the database
#[async_trait]
pub trait UsernameExists: ops::GetConnection {
    /// Database specific error-type
    type Error: std::error::Error;
    /// check if a username exists in the database
    async fn username_exists(
        &self,
        username: &str,
    ) -> DBResult<bool, <Self as UsernameExists>::Error>;
}

/// payload to update a username in database
pub struct UpdateUsernamePayload<'a> {
    /// old usename
    pub old_username: &'a str,
    /// new username
    pub new_username: &'a str,
}

/// update username in database
#[async_trait]
pub trait UpdateUsername: ops::GetConnection {
    /// Database specific error-type
    type Error: std::error::Error;
    /// update username in database
    async fn update_username(
        &self,
        payload: &UpdateUsernamePayload,
    ) -> DBResult<(), <Self as UpdateUsername>::Error>;
}

/// update user secret in database
#[async_trait]
pub trait UpdateSecret: ops::GetConnection {
    /// Database specific error-type
    type Error: std::error::Error;
    /// update secret in database
    async fn update_secret(
        &self,
        username: &str,
        secret: &str,
    ) -> DBResult<(), <Self as UpdateSecret>::Error>;
}

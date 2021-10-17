//! Account operations: update email, password and account secrets, etc.
use crate::dev::*;

/// Top-level trait grouping all account management operations
pub trait Account:
    UpdateEmail + EmailExists + DeleteAccount + UpdatePassword + UpdateUsername + UsernameExists
{
}

pub use crate::auth::login::Creds;

/// payload to update email in the database
#[derive(Clone, Debug)]
pub struct UpdateEmailPayload {
    /// name of the user who's email is to be updated
    pub name: String,
    /// new email
    pub email: String,
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
pub struct UpdateUsernamePayload {
    /// old usename
    pub old_username: String,
    /// new username
    pub new_username: String,
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

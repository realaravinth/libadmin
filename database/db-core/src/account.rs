use async_trait::async_trait;

use crate::errors::*;

pub trait Account:
    UpdateEmail + EmailExists + DeleteAccount + UpdatePassword + UpdateUsername + UsernameExists
{
}

use crate::auth::login::Creds;
use crate::DBConn;

#[derive(Clone, Debug)]
pub struct UpdateEmailPayload {
    pub name: String,
    pub email: String,
}

#[async_trait]
pub trait UpdateEmail {
    type Error: std::error::Error;
    async fn username_login<C: DBConn>(
        conn: C,
        payload: &UpdateEmailPayload,
    ) -> DBResult<(), Self::Error>;
}

#[async_trait]
pub trait UpdatePassword {
    type Error: std::error::Error;
    async fn update_password<C: DBConn>(conn: C, payload: &Creds) -> DBResult<(), Self::Error>;
}

#[async_trait]
pub trait EmailExists {
    type Error: std::error::Error;
    async fn email_exists<C: DBConn>(conn: C, email: &str) -> DBResult<bool, Self::Error>;
}

#[async_trait]
pub trait DeleteAccount {
    type Error: std::error::Error;
    async fn delete_account<C: DBConn>(conn: C, username: &str) -> DBResult<(), Self::Error>;
}

#[async_trait]
pub trait UsernameExists {
    type Error: std::error::Error;
    async fn username_exists<C: DBConn>(conn: C, username: &str) -> DBResult<bool, Self::Error>;
}

pub struct UpdateUsernamePayload {
    pub old_username: String,
    pub new_username: String,
}

#[async_trait]
pub trait UpdateUsername {
    type Error: std::error::Error;
    async fn delete_account<C: DBConn>(
        conn: C,
        payload: &UpdateUsernamePayload,
    ) -> DBResult<(), Self::Error>;
}

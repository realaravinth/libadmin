use async_trait::async_trait;

use crate::errors::*;

pub trait Auth: login::Login + register::Register {}

use crate::DBConn;
pub mod login {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct Creds {
        pub username: String,
        pub password: String,
    }

    #[derive(Clone, Debug)]
    pub struct Password {
        pub password: String,
    }

    pub trait Login: EmailLogin + UsernameLogin {}

    #[async_trait]
    pub trait EmailLogin {
        type Error: std::error::Error;
        async fn email_login<C: DBConn>(conn: C, email: &str) -> DBResult<Creds, Self::Error>;
    }

    #[async_trait]
    pub trait UsernameLogin {
        type Error: std::error::Error;
        async fn username_login<C: DBConn>(
            conn: C,
            username: &str,
        ) -> DBResult<Password, Self::Error>;
    }
}

pub mod register {
    use super::*;

    pub trait Register: EmailRegister + UsernameRegister {}

    pub struct EmailRegisterPayload {
        pub username: String,
        pub password: String,
        pub email: String,
        pub secret: String,
    }

    pub struct UsernameRegisterPayload {
        pub username: String,
        pub password: String,
        pub secret: String,
    }

    #[async_trait]
    pub trait EmailRegister {
        type Error: std::error::Error;
        async fn email_register<C: DBConn>(
            conn: C,
            payload: &EmailRegisterPayload,
        ) -> DBResult<(), Self::Error>;
    }

    #[async_trait]
    pub trait UsernameRegister {
        type Error: std::error::Error;
        async fn username_login<C: DBConn>(
            conn: C,
            payload: &UsernameRegisterPayload,
        ) -> DBResult<(), Self::Error>;
    }
}

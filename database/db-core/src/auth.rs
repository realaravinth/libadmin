//! Authentication and registration operations
use crate::dev::*;

/// Top-level trait grouping authentication and registration operations
pub trait Auth: login::Login + register::Register {}

pub mod login {
    //! Login mechanisms and datastructures
    use super::*;

    /// data structure describing credentials of a user
    #[derive(Clone, Debug)]
    pub struct Creds {
        /// username
        pub username: String,
        /// password
        pub password: String,
    }

    /// data structure containing only a password field
    #[derive(Clone, Debug)]
    pub struct Password {
        /// password
        pub password: String,
    }

    /// Top-level trait grouping different login mechanisms
    pub trait Login: EmailLogin + UsernameLogin {}

    /// Login using email as user-identifier
    #[async_trait]
    pub trait EmailLogin: GetConnection {
        /// database specific error-type
        type Error: std::error::Error;
        /// login with email as user-identifier
        async fn email_login(&self, email: &str) -> DBResult<Creds, <Self as EmailLogin>::Error>;
    }

    /// Login using username as user-identifier
    #[async_trait]
    pub trait UsernameLogin: GetConnection {
        /// database specific error-type
        type Error: std::error::Error;
        /// login with username as user-identifier
        async fn username_login(
            &self,
            username: &str,
        ) -> DBResult<Password, <Self as UsernameLogin>::Error>;
    }
}

pub mod register {
    //! registration operations
    use super::*;

    /// Top-level traits group all registration mechanisms
    pub trait Register: EmailRegister + UsernameRegister {}

    /// payload to register a user with username _and_ email
    pub struct EmailRegisterPayload<'a> {
        /// username of new user
        pub username: &'a str,
        /// password of new user
        pub password: &'a str,
        /// password of new user
        pub email: &'a str,
        /// a randomly generated secret associated with an account
        pub secret: &'a str,
    }

    /// payload to register a user with only username
    pub struct UsernameRegisterPayload<'a> {
        /// username provided during registration
        pub username: &'a str,
        /// password of new user
        pub password: &'a str,
        /// a randomly generated secret associated with an account
        pub secret: &'a str,
    }

    #[async_trait]
    /// register user with username _and_ email
    pub trait EmailRegister: GetConnection {
        /// database specific error-type
        type Error: std::error::Error;
        /// username _and_ email is available during registration
        async fn email_register(
            &self,
            payload: &EmailRegisterPayload,
        ) -> DBResult<(), <Self as EmailRegister>::Error>;
    }

    #[async_trait]
    /// register with username only
    pub trait UsernameRegister: GetConnection {
        /// database specific error-type
        type Error: std::error::Error;
        /// register with username
        async fn username_register(
            &self,
            payload: &UsernameRegisterPayload,
        ) -> DBResult<(), <Self as UsernameRegister>::Error>;
    }
}

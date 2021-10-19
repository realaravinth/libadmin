//! represents all the ways a trait can fail using this crate
use derive_more::{Display, Error as DeriveError};

/// Error data structure grouping various error subtypes
#[derive(Debug, Display, DeriveError)]
pub enum DBError {
    /// username is already taken
    #[display(fmt = "Username not available")]
    DuplicateUsername,

    /// user secret is already taken
    #[display(fmt = "User secret not available")]
    DuplicateSecret,

    /// email is already taken
    #[display(fmt = "Email not available")]
    DuplicateEmail,

    /// Account with specified characteristics not found
    #[display(fmt = "Account with specified characteristics not found")]
    AccountNotFound,

    /// errors that are specific to a database implementation
    #[display(fmt = "Database error: {:?}", _0)]
    DBError(#[error(not(source))] String),
}

/// Generic result data structure
pub type DBResult<V> = std::result::Result<V, DBError>;

//! represents all the ways a trait can fail using this crate
use std::error::Error;

use derive_more::{Display, Error as DeriveError};

/// Error data structure grouping various error subtypes
#[derive(Debug, Display, PartialEq, DeriveError)]
pub enum DBError<DB>
where
    DB: Error,
{
    /// username is already taken
    #[display(fmt = "Username not available")]
    DuplicateUsername,

    /// user secret is already taken
    #[display(fmt = "User secret not available")]
    DuplicateSecret,

    /// email is already taken
    #[display(fmt = "Email not available")]
    DuplicateEmail,

    /// errors that are specific to a database implementation
    #[display(fmt = "Database error: {:?}", _0)]
    DBError(DB),
}

/// Generic result data structure
pub type DBResult<V, E> = std::result::Result<V, DBError<E>>;

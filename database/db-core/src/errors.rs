use std::error::Error;

use derive_more::{Display, Error as DeriveError};

#[derive(Debug, Display, PartialEq, DeriveError)]
#[cfg(not(tarpaulin_include))]
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

    /// DB Error
    #[display(fmt = "Database error: {:?}", _0)]
    DBError(DB),
}

#[cfg(not(tarpaulin_include))]
pub type DBResult<V, E> = std::result::Result<V, DBError<E>>;

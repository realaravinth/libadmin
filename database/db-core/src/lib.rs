#![deny(missing_docs)]
//! # `libadmin` database operations
//!
//! Traits and datastructures used in libadmin to interact with database.
//!
//! To use an unsupported database with libadmin, traits present within this crate should be
//! implemented.
//!
//!
//! ## Organisation
//!
//! Database functionallity is divided accross various modules:
//!
//! - [account](crate::account): account management operations(updates, deletions, etc.)
//! - [auth](crate::auth): registration and authentication operations
//! - [errors](crate::auth): error data structures used in this crate
//! - [ops](crate::ops): meta operations like connection pool creation, migrations and getting
//! connection from pool
pub mod account;
pub mod auth;
pub mod errors;
pub mod ops;

pub use ops::GetConnection;

/// Top level trait describing all libadmin database operations
pub trait LibAdminDatabase: ops::DBOps + auth::Auth + account::Account {}

pub mod prelude {
    //! useful imports for users working with a supported database
    pub use super::account::*;
    pub use super::auth::login::*;
    pub use super::auth::register::*;
    pub use super::auth::*;
    pub use super::errors::*;
    pub use super::ops::*;
    pub use super::*;
}

pub mod dev {
    //! useful imports for supporting a new database
    pub use super::prelude::*;
    pub use async_trait::async_trait;
}

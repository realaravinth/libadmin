use std::borrow::Cow;

use db_core::dev::*;
use sqlx::Error;

pub fn map_register_err(e: Error) -> DBError {
    if let Error::Database(err) = e {
        if err.code() == Some(Cow::from("2067")) {
            let msg = err.message();
            if msg.contains("admin_users.username") {
                DBError::DuplicateUsername
            } else if msg.contains("admin_users.email") {
                DBError::DuplicateEmail
            } else if msg.contains("admin_users.secret") {
                DBError::DuplicateSecret
            } else {
                DBError::DBError(format!("{:?}", Error::Database(err)))
            }
        } else {
            DBError::DBError(format!("{:?}", Error::Database(err)))
        }
    } else {
        DBError::DBError(format!("{:?}", e))
    }
}

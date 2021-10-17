//use std::convert::From;
//
//use db_core::errors::DBError;
//use sqlx::Error as SqlxError;
//
//impl From<SqlxError> for DBError<SqlxError> {
//    #[cfg(not(tarpaulin_include))]
//    fn from(e: SqlxError) -> Self {
//        DBError::DBError(e)
//    }
//}

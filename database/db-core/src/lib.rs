pub mod auth;
pub mod errors;

use errors::*;

/// Get database connection
pub trait DBConn {
    /// Database connection type
    type Conn;
    type Error: std::error::Error;
    fn conn(&self) -> DBResult<Self::Conn, Self::Error>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

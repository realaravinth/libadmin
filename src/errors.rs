/*
 * Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
//! represents all the ways a trait can fail using this crate
use std::convert::From;

use argon2_creds::errors::CredsError;
use db_core::errors::DBError;
use derive_more::{Display, Error};
use url::ParseError;
use validator::ValidationErrors;

#[derive(Debug, Display, PartialEq, Error)]
#[cfg(not(tarpaulin_include))]
/// Error data structure grouping various error subtypes
pub enum ServiceError {
    #[display(fmt = "internal server error")]
    InternalServerError,

    #[display(
        fmt = "This server is is closed for registration. Contact admin if this is unexpecter"
    )]
    ClosedForRegistration,

    #[display(fmt = "The value you entered for email is not an email")] //405j
    NotAnEmail,
    #[display(fmt = "The value you entered for URL is not a URL")] //405j
    NotAUrl,
    #[display(fmt = "The value you entered for ID is not a valid ID")] //405j
    NotAnId,
    #[display(fmt = "URL too long, maximum length can't be greater then 2048 characters")] //405
    URLTooLong,

    #[display(fmt = "Wrong password")]
    WrongPassword,
    #[display(fmt = "Account not found")]
    AccountNotFound,

    /// when the value passed contains profainity
    #[display(fmt = "Can't allow profanity in usernames")]
    ProfainityError,
    /// when the value passed contains blacklisted words
    /// see [blacklist](https://github.com/shuttlecraft/The-Big-Username-Blacklist)
    #[display(fmt = "Username contains blacklisted words")]
    BlacklistError,
    /// when the value passed contains characters not present
    /// in [UsernameCaseMapped](https://tools.ietf.org/html/rfc8265#page-7)
    /// profile
    #[display(fmt = "username_case_mapped violation")]
    UsernameCaseMappedError,

    #[display(fmt = "Passsword too short")]
    PasswordTooShort,
    #[display(fmt = "Username too long")]
    PasswordTooLong,
    #[display(fmt = "Passwords don't match")]
    PasswordsDontMatch,

    /// when the a username is already taken
    #[display(fmt = "Username not available")]
    UsernameTaken,

    /// email is already taken
    #[display(fmt = "Email not available")]
    EmailTaken,
}

/// HTTP Status code of errors
pub trait ErrorStatusCode {
    fn status_code(&self) -> u16;
}

impl ErrorStatusCode for ServiceError {
    #[cfg(not(tarpaulin_include))]
    fn status_code(&self) -> u16 {
        match self {
            ServiceError::ClosedForRegistration => 403, //FORBIDDEN,
            ServiceError::InternalServerError => 500,   // INTERNAL SERVER ERROR
            ServiceError::NotAnEmail => 400,            //BADREQUEST,
            ServiceError::NotAUrl => 400,               //BADREQUEST,
            ServiceError::NotAnId => 400,               //BADREQUEST,
            ServiceError::URLTooLong => 400,            //BADREQUEST,
            ServiceError::WrongPassword => 401,         //UNAUTHORIZED,
            ServiceError::AccountNotFound => 400,       //NOT FOUND,

            ServiceError::ProfainityError => 400, //BADREQUEST,
            ServiceError::BlacklistError => 400,  //BADREQUEST,
            ServiceError::UsernameCaseMappedError => 400, //BADREQUEST,

            ServiceError::PasswordTooShort => 400, //BADREQUEST,
            ServiceError::PasswordTooLong => 400,  //BADREQUEST,
            ServiceError::PasswordsDontMatch => 400, //BADREQUEST,

            ServiceError::UsernameTaken => 400, //BADREQUEST,
            ServiceError::EmailTaken => 400,    //BADREQUEST,
        }
    }
}

impl From<CredsError> for ServiceError {
    #[cfg(not(tarpaulin_include))]
    fn from(e: CredsError) -> ServiceError {
        match e {
            CredsError::UsernameCaseMappedError => ServiceError::UsernameCaseMappedError,
            CredsError::ProfainityError => ServiceError::ProfainityError,
            CredsError::BlacklistError => ServiceError::BlacklistError,
            CredsError::NotAnEmail => ServiceError::NotAnEmail,
            CredsError::Argon2Error(_) => ServiceError::InternalServerError,
            CredsError::PasswordTooLong => ServiceError::PasswordTooLong,
            CredsError::PasswordTooShort => ServiceError::PasswordTooShort,
        }
    }
}

impl From<ValidationErrors> for ServiceError {
    #[cfg(not(tarpaulin_include))]
    fn from(_: ValidationErrors) -> ServiceError {
        ServiceError::NotAnEmail
    }
}

impl From<ParseError> for ServiceError {
    #[cfg(not(tarpaulin_include))]
    fn from(_: ParseError) -> ServiceError {
        ServiceError::NotAUrl
    }
}

impl<E: std::error::Error> From<DBError<E>> for ServiceError {
    fn from(e: DBError<E>) -> Self {
        log::error!("{:?}", e);
        match e {
            DBError::DBError(_) => ServiceError::InternalServerError,
            DBError::DuplicateEmail => ServiceError::EmailTaken,
            DBError::DuplicateUsername => ServiceError::UsernameTaken,
            DBError::AccountNotFound => ServiceError::AccountNotFound,
            DBError::DuplicateSecret => ServiceError::InternalServerError,
        }
    }
}

/// Generic result data structure
#[cfg(not(tarpaulin_include))]
pub type ServiceResult<V> = std::result::Result<V, ServiceError>;

//#[derive(Debug, Display, PartialEq, Error)]
//#[cfg(not(tarpaulin_include))]
//pub enum PageError {
//    #[display(fmt = "Something weng wrong: Internal server error")]
//    InternalServerError,
//
//    #[display(fmt = "{}", _0)]
//    ServiceError(ServiceError),
//}
//
//#[cfg(not(tarpaulin_include))]
//impl From<sqlx::Error> for PageError {
//    #[cfg(not(tarpaulin_include))]
//    fn from(_: sqlx::Error) -> Self {
//        PageError::InternalServerError
//    }
//}
//
//#[cfg(not(tarpaulin_include))]
//impl From<ServiceError> for PageError {
//    #[cfg(not(tarpaulin_include))]
//    fn from(e: ServiceError) -> Self {
//        PageError::ServiceError(e)
//    }
//}
//
//impl ResponseError for PageError {
//    fn error_response(&self) -> HttpResponse {
//        use crate::PAGES;
//        match self.status_code() {
//            StatusCode::INTERNAL_SERVER_ERROR => HttpResponse::Found()
//                .append_header((header::LOCATION, PAGES.errors.internal_server_error))
//                .finish(),
//            _ => HttpResponse::Found()
//                .append_header((header::LOCATION, PAGES.errors.unknown_error))
//                .finish(),
//        }
//    }
//
//    #[cfg(not(tarpaulin_include))]
//    fn status_code(&self) -> StatusCode {
//        match self {
//            PageError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
//            PageError::ServiceError(e) => e.status_code(),
//        }
//    }
//}
//
//#[cfg(not(tarpaulin_include))]
//pub type PageResult<V> = std::result::Result<V, PageError>;
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//    use crate::PAGES;
//
//    #[test]
//    fn error_works() {
//        let resp: HttpResponse = PageError::InternalServerError.error_response();
//        assert_eq!(resp.status(), StatusCode::FOUND);
//        let headers = resp.headers();
//        assert_eq!(
//            headers.get(header::LOCATION).unwrap(),
//            PAGES.errors.internal_server_error
//        );
//    }
//}

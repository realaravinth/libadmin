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
//! V1 API Routes

/// constant [Routes](Routes) instance
pub const ROUTES: Routes = Routes::new();

/// Authentication routes
pub struct Auth {
    /// logout route
    pub logout: &'static str,
    /// login route
    pub login: &'static str,
    /// registration route
    pub register: &'static str,
}
impl Auth {
    /// create new instance of Authentication route
    pub const fn new() -> Auth {
        let login = "/api/v1/signin";
        let logout = "/logout";
        let register = "/api/v1/signup";
        Auth {
            logout,
            login,
            register,
        }
    }
}

/// Account management routes
pub struct Account {
    /// delete account route
    pub delete: &'static str,
    /// route to check if an email exists
    pub email_exists: &'static str,
    /// route to fetch account secret
    pub get_secret: &'static str,
    /// route to update a user's email
    pub update_email: &'static str,
    ///    route to update password
    pub update_password: &'static str,
    ///    route to update secret
    pub update_secret: &'static str,
    ///    route to check if a username is already registered
    pub username_exists: &'static str,
    ///    route to change username
    pub update_username: &'static str,
}

impl Account {
    /// create a new instance of [Account][Account] routes
    pub const fn new() -> Account {
        let get_secret = "/api/v1/account/secret/get";
        let update_secret = "/api/v1/account/secret/update";
        let delete = "/api/v1/account/delete";
        let email_exists = "/api/v1/account/email/exists";
        let username_exists = "/api/v1/account/username/exists";
        let update_username = "/api/v1/account/username/update";
        let update_email = "/api/v1/account/email/update";
        let update_password = "/api/v1/account/password/update";
        Account {
            delete,
            email_exists,
            get_secret,
            update_email,
            update_password,
            update_secret,
            username_exists,
            update_username,
        }
    }
}

/// Top-level routes data structure for V1 AP1
pub struct Routes {
    /// Authentication routes
    pub auth: Auth,
    /// Account routes
    pub account: Account,
}

impl Routes {
    /// create new instance of Routes
    const fn new() -> Routes {
        Routes {
            auth: Auth::new(),
            account: Account::new(),
        }
    }
}

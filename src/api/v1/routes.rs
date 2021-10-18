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
pub const ROUTES: Routes = Routes::new();

pub struct Auth {
    pub logout: &'static str,
    pub login: &'static str,
    pub register: &'static str,
}
impl Auth {
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

pub struct Account {
    pub delete: &'static str,
    pub email_exists: &'static str,
    pub get_secret: &'static str,
    pub update_email: &'static str,
    pub update_password: &'static str,
    pub update_secret: &'static str,
    pub username_exists: &'static str,
    pub update_username: &'static str,
}

impl Account {
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

pub struct Routes {
    pub auth: Auth,
    pub account: Account,
}

impl Routes {
    const fn new() -> Routes {
        Routes {
            auth: Auth::new(),
            account: Account::new(),
        }
    }
}

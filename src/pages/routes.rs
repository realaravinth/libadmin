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
use super::auth::routes::Auth;
use super::errors::routes::Errors;
pub const ROUTES: Routes = Routes::new();

pub struct Routes {
    pub home: &'static str,
    pub auth: Auth,
    pub errors: Errors,
    pub about: &'static str,
    pub sitemap: &'static str,
    pub thanks: &'static str,
    pub donate: &'static str,
    pub security: &'static str,
    pub privacy: &'static str,
}

impl Routes {
    const fn new() -> Routes {
        let auth = Auth::new();
        let home = auth.login;
        Routes {
            auth,
            home,
            errors: Errors::new(),
            about: "/about",
            sitemap: "/sitemap.xml",
            thanks: "/thanks",
            donate: "/donate",
            security: "/security",
            privacy: "/privacy-policy",
        }
    }

    //    pub const fn get_sitemap() -> [&'static str; 3] {
    //        let a = Auth::get_sitemap();
    //        let p = Panel::get_sitemap();
    //        [a[0], a[1], p[0]] //, p[1], p[2], p[3], p[4]]
    //    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn sitemap_works() {
//        Routes::get_sitemap();
//    }
//}

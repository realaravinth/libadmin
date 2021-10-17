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
use sailfish::TemplateOnce;

use crate::pages::errors::ErrorPage;

#[derive(Clone, TemplateOnce)]
#[template(path = "auth/sudo/index.html")]
pub struct SudoPage<'a> {
    url: &'a str,
    title: &'a str,
    error: Option<ErrorPage<'a>>,
}

pub const PAGE: &str = "Confirm Access";

impl<'a> SudoPage<'a> {
    pub fn new(url: &'a str, title: &'a str) -> Self {
        Self {
            url,
            title,
            error: None,
        }
    }

    pub fn set_err(&mut self, err_title: &'a str, message: &'a str) {
        self.error = Some(ErrorPage::new(err_title, message));
    }
}

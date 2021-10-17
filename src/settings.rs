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
use derive_builder::Builder;
use serde::Deserialize;
use url::Url;

use crate::api::v1::get_random;

#[derive(Debug, Clone, Deserialize, Builder)]
pub struct Server {
    pub domain: String,
    pub cookie_secret: String,
    //    pub proxy_has_tls: bool,
}

#[derive(Debug, Clone, Deserialize, Builder)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct Database {
    pub url: String,
    pub pool: u32,
}

impl Default for Server {
    fn default() -> Self {
        let domain = "localhost".into();
        let cookie_secret = get_random(40);

        Self {
            domain,
            cookie_secret,
        }
    }
}

impl Default for Database {
    fn default() -> Self {
        let url = std::env::var("DATABASE_URL").expect("set DATABASE_URL");
        let pool = 4;

        Self { url, pool }
    }
}

impl DatabaseBuilder {
    fn validate(&self) -> Result<(), String> {
        Url::parse(self.url.as_ref().unwrap()).map_err(|_| "Enter valid URL for DATABASE_URL")?;
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Builder)]
pub struct Settings {
    pub allow_registration: bool,
    pub allow_demo: bool,
    pub database: Database,
    pub server: Server,
}

impl Default for Settings {
    fn default() -> Self {
        let allow_registration = false;
        let allow_demo = false;
        let database = Database::default();
        let server = Server::default();
        Self {
            allow_demo,
            allow_registration,
            database,
            server,
        }
    }
}

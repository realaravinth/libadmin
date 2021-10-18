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
//! App settings
use derive_builder::Builder;
use serde::Deserialize;
use url::Url;

use crate::api::v1::get_random;

/// Server data
#[derive(Debug, Clone, Deserialize, Builder)]
pub struct Server {
    /// Domain name of the server
    pub domain: String,
    /// Used to sign cookies. Provide a randomly generated string of atleast 32 chars long.
    pub cookie_secret: String,
    //    pub proxy_has_tls: bool,
}

/// Database settings
#[derive(Debug, Clone, Deserialize, Builder)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct Database {
    /// URL of the database
    pub url: String,
    /// Pool size: maximum number of connections to open
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

/// Top-level settings data structure
#[derive(Debug, Clone, Deserialize, Builder)]
pub struct Settings {
    /// enable registration
    pub allow_registration: bool,
    /// enable demo user
    pub allow_demo: bool,
    /// database configuration
    pub database: Database,
    /// Domain name and cookie secret
    pub server: Server,
}

impl Default for Settings {
    fn default() -> Self {
        let allow_registration = true;
        let allow_demo = true;
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

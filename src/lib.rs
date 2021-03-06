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
#![deny(missing_docs)]
//! # `libadmin`
//! Authentication, session-management and access control mechanism for web servers built in Rust
use std::sync::Arc;
use std::thread;

use argon2_creds::{Config, ConfigBuilder, PasswordPolicy};

pub mod api;
//pub mod demo;
pub mod errors;
mod settings;
#[cfg(test)]
mod tests;

pub use api::v1::ROUTES as V1_API_ROUTES;
pub use settings::Settings;

/// Default cache age for static assets
pub const CACHE_AGE: u32 = 604800;

/// App data
pub struct Data {
    /// database pool
    /// credential-procession policy
    pub creds: Config,

    /// settings
    pub settings: Settings,
}

//impl<T: LibAdminDatabase> Marker for Data<T> {
//    type DB = Box<dyn LibAdminDatabase>;
//    fn get_settings(&self) -> &Settings {
//        &self.settings
//    }
//
//    fn get_db(&self) -> &T {
//        &self.db
//    }
//}

impl Data {
    /// Get credential-processing policy
    pub fn get_creds() -> Config {
        ConfigBuilder::default()
            .username_case_mapped(true)
            .profanity(true)
            .blacklist(true)
            .password_policy(PasswordPolicy::default())
            .build()
            .unwrap()
    }

    #[cfg(not(tarpaulin_include))]
    /// create new instance of app data
    pub async fn new(settings: Settings) -> Arc<Self> {
        //        #[cfg(test)]
        //        crate::tests::init();

        let creds = Self::get_creds();
        let c = creds.clone();

        #[allow(unused_variables)]
        let init = thread::spawn(move || {
            log::info!("Initializing credential manager");
            c.init();
            log::info!("Initialized credential manager");
        });

        let data = Data { creds, settings };

        #[cfg(not(debug_assertions))]
        init.join().unwrap();

        Arc::new(data)
    }
}

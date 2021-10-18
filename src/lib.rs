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
use db_core::prelude::*;
use once_cell::sync::OnceCell;

mod api;
pub mod demo;
mod errors;
mod settings;
#[cfg(test)]
mod tests;

pub use api::v1::ROUTES as V1_API_ROUTES;
pub use settings::Settings;

/// Settings
pub static SETTINGS: OnceCell<Settings> = OnceCell::new();

/// Default cache age for static assets
pub const CACHE_AGE: u32 = 604800;

/// load settings
pub fn init(settings: Settings) {
    SETTINGS.get_or_init(|| settings);
}

/// App data
pub struct Data<T: LibAdminDatabase> {
    /// databse pool
    pub db: T,
    /// credential-procession policy
    pub creds: Config,
}

impl<T: LibAdminDatabase> Data<T> {
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
    pub async fn new<V, E>(db: V) -> Arc<Self>
    where
        V: Connect<Pool = T, Error = E>,
        E: std::fmt::Debug + std::error::Error,
    {
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

        let db = db.connect().await.unwrap();

        let data = Data { db, creds };

        #[cfg(not(debug_assertions))]
        init.join().unwrap();

        Arc::new(data)
    }
}

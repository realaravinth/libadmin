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
//! App data: database connections, etc.
use std::sync::Arc;
use std::thread;

use argon2_creds::{Config, ConfigBuilder, PasswordPolicy};
use db_core::prelude::*;

/// App data
pub struct Data<P: LibAdminDatabase> {
    /// databse pool
    pub db: P,
    pub creds: Config,
}

impl<P: GetConnection> Data<P> {
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
    pub async fn new<C, E, T>(db: T) -> Arc<Self>
    where
        T: Connect<Config = C, Pool = P, Error = E>,
    {
        #[cfg(test)]
        crate::tests::init();

        let settings = crate::SETTINGS.get().unwrap();
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

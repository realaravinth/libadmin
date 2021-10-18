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
use lazy_static::lazy_static;

mod api;
mod data;
pub mod demo;
mod errors;
mod settings;
mod static_assets;

pub use crate::data::Data;
pub use api::v1::ROUTES as V1_API_ROUTES;
pub use settings::Settings;
pub use static_assets::static_files::assets;

use static_assets::FileMap;

use once_cell::sync::OnceCell;
pub static SETTINGS: OnceCell<Settings> = OnceCell::new();

lazy_static! {
    //pub static ref SETTINGS: Settings = Settings::new().unwrap();
    pub static ref FILES: FileMap = FileMap::new();

    pub static ref CSS: &'static str =
        FILES.get("./static/cache/bundle/css/main.css").unwrap();
}

pub const CACHE_AGE: u32 = 604800;

//pub type AppData<T> = actix_web::web::Data<Arc<crate::data::Data<T>>>;

pub fn init(settings: Settings) {
    let _ = SETTINGS.set(settings);
}

//#[cfg(not(tarpaulin_include))]
//pub fn get_json_err() -> JsonConfig {
//    JsonConfig::default().error_handler(|err, _| {
//        log::debug!("JSON deserialization error: {:?}", &err);
//        InternalError::new(err, StatusCode::BAD_REQUEST).into()
//    })
//}
//
//#[cfg(not(tarpaulin_include))]
//pub fn get_identity_service() -> IdentityService<CookieIdentityPolicy> {
//    let settings = SETTINGS.get().unwrap();
//    let cookie_secret = &settings.server.cookie_secret;
//    IdentityService::new(
//        CookieIdentityPolicy::new(cookie_secret.as_bytes())
//            .name("Authorization")
//            //TODO change cookie age
//            .max_age_secs(216000)
//            .domain(&settings.server.domain)
//            .secure(false),
//    )
//}

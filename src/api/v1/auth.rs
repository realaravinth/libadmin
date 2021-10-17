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

use actix_identity::Identity;
use actix_web::http::header;
use actix_web::{web, HttpResponse, Responder};
use db_core::prelude::*;
use serde::{Deserialize, Serialize};

use super::get_random;
use super::RedirectQuery;
use crate::errors::*;
use crate::AppData;

pub mod routes {
    use actix_auth_middleware::Authentication;
    use actix_auth_middleware::GetLoginRoute;

    pub struct Auth {
        pub logout: &'static str,
        pub login: &'static str,
        pub register: &'static str,
    }

    pub fn get_auth_middleware() -> Authentication<Auth> {
        Authentication::with_identity(crate::V1_API_ROUTES.auth)
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

    impl GetLoginRoute for Auth {
        fn get_login_route(&self, src: Option<&str>) -> String {
            if let Some(redirect_to) = src {
                format!(
                    "{}?redirect_to={}",
                    self.login,
                    urlencoding::encode(redirect_to)
                )
            } else {
                self.register.to_string()
            }
        }
    }
}

pub mod runners {
    use super::*;

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Register {
        pub username: String,
        pub password: String,
        pub confirm_password: String,
        pub email: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Login {
        // login accepts both username and email under "username field"
        // TODO update all instances where login is used
        pub login: String,
        pub password: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Password {
        pub password: String,
    }

    /// returns Ok(()) when everything checks out and the user is authenticated. Erros otherwise
    pub async fn login_runner<T>(payload: &Login, data: &AppData<T>) -> ServiceResult<String> {
        use argon2_creds::Config;

        let verify = |stored: &str, received: &str| {
            if Config::verify(stored, received)? {
                Ok(())
            } else {
                Err(ServiceError::WrongPassword)
            }
        };

        let res = if payload.login.contains('@') {
            data.db.email_login(&payload.login).await;
        } else {
            data.db.username_login(&payload.login).await;
        };
        match res {
            Ok(s) => {
                verify(&s.password, &payload.password)?;
                Ok(payload.login.clone())
            }
            Err(DBError::AccountNotFound) => Err(ServiceError::AccountNotFound),
            Err(_) => Err(ServiceError::InternalServerError),
        }
    }

    pub async fn register_runner<T>(payload: &Register, data: &AppData<T>) -> ServiceResult<()> {
        if !crate::SETTINGS.get().unwrap().allow_registration {
            return Err(ServiceError::ClosedForRegistration);
        }

        if payload.password != payload.confirm_password {
            return Err(ServiceError::PasswordsDontMatch);
        }
        let username = data.creds.username(&payload.username)?;
        let hash = data.creds.password(&payload.password)?;

        if let Some(email) = &payload.email {
            data.creds.email(email)?;
        }

        let mut secret;

        if let Some(email) = &payload.email {
            loop {
                secret = get_random(32);

                let mut db_payload = EmailRegisterPayload {
                    secret,
                    username,
                    password: hash,
                    email: email.to_owned(),
                };

                match data.db.email_register(&db_payload).await {
                    Ok(_) => break,
                    Err(DBError::DuplicateSecret) => continue,
                    Err(e) => return Err(e.into()),
                }
            }
        } else {
            loop {
                secret = get_random(32);

                let mut db_payload = UsernameRegisterPayload {
                    secret,
                    username,
                    password: hash,
                };

                match data.db.username_register(&db_payload).await {
                    Ok(_) => break,
                    Err(DBError::DuplicateSecret) => continue,
                    Err(e) => return Err(e.into()),
                }
            }
        }
        Ok(())
    }
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
    cfg.service(signout);
}
#[my_codegen::post(path = "crate::V1_API_ROUTES.auth.register")]
async fn register<T>(
    payload: web::Json<runners::Register>,
    data: AppData<T>,
) -> ServiceResult<impl Responder> {
    runners::register_runner(&payload, &data).await?;
    Ok(HttpResponse::Ok())
}

#[my_codegen::post(path = "crate::V1_API_ROUTES.auth.login")]
async fn login<T>(
    id: Identity,
    payload: web::Json<runners::Login>,
    query: web::Query<RedirectQuery>,
    data: AppData<T>,
) -> ServiceResult<impl Responder> {
    let payload = payload.into_inner();
    let username = runners::login_runner(&payload, &data).await?;
    id.remember(username);
    let query = query.into_inner();
    if let Some(redirect_to) = query.redirect_to {
        Ok(HttpResponse::Found()
            .insert_header((header::LOCATION, redirect_to))
            .finish())
    } else {
        Ok(HttpResponse::Ok().into())
    }
}

#[my_codegen::get(
    path = "crate::V1_API_ROUTES.auth.logout",
    wrap = "crate::get_auth_middleware()"
)]
async fn signout(id: Identity) -> impl Responder {
    use actix_auth_middleware::GetLoginRoute;

    if id.identity().is_some() {
        id.forget();
    }
    HttpResponse::Found()
        .append_header((
            header::LOCATION,
            crate::V1_API_ROUTES.auth.get_login_route(None),
        ))
        .finish()
}

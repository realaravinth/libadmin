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
use actix_web::{web, HttpResponse, Responder};
use argon2_creds::Config;
use serde::{Deserialize, Serialize};
use sqlx::Error::RowNotFound;

use crate::api::v1::auth::runners::Password;
use crate::errors::*;
use crate::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChangePasswordReqest {
    pub password: String,
    pub new_password: String,
    pub confirm_new_password: String,
}

pub struct UpdatePassword {
    pub new_password: String,
    pub confirm_new_password: String,
}

impl From<ChangePasswordReqest> for UpdatePassword {
    fn from(s: ChangePasswordReqest) -> Self {
        UpdatePassword {
            new_password: s.new_password,
            confirm_new_password: s.confirm_new_password,
        }
    }
}

async fn update_password_runner(
    user: &str,
    update: UpdatePassword,
    data: &Data,
) -> ServiceResult<()> {
    if update.new_password != update.confirm_new_password {
        return Err(ServiceError::PasswordsDontMatch);
    }

    let new_hash = data.creds.password(&update.new_password)?;

    sqlx::query!(
        "UPDATE admin_users set password = $1
        WHERE name = $2",
        &new_hash,
        &user,
    )
    .execute(&data.db)
    .await?;

    Ok(())
}

#[my_codegen::post(
    path = "crate::V1_API_ROUTES.account.update_password",
    wrap = "crate::get_auth_middleware()"
)]
async fn update_user_password(
    id: Identity,
    data: AppData,
    payload: web::Json<ChangePasswordReqest>,
) -> ServiceResult<impl Responder> {
    if payload.new_password != payload.confirm_new_password {
        return Err(ServiceError::PasswordsDontMatch);
    }

    let username = id.identity().unwrap();

    let rec = sqlx::query_as!(
        Password,
        r#"SELECT password  FROM admin_users WHERE name = ($1)"#,
        &username,
    )
    .fetch_one(&data.db)
    .await;

    match rec {
        Ok(s) => {
            if Config::verify(&s.password, &payload.password)? {
                let update: UpdatePassword = payload.into_inner().into();
                update_password_runner(&username, update, &data).await?;
                Ok(HttpResponse::Ok())
            } else {
                Err(ServiceError::WrongPassword)
            }
        }
        Err(RowNotFound) => Err(ServiceError::AccountNotFound),
        Err(_) => Err(ServiceError::InternalServerError),
    }
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(update_user_password);
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::http::StatusCode;
    use actix_web::test;

    use crate::api::v1::ROUTES;
    use crate::data::Data;
    use crate::tests::*;

    #[actix_rt::test]
    async fn update_password_works() {
        const NAME: &str = "updatepassuser";
        const PASSWORD: &str = "longpassword2";
        const EMAIL: &str = "updatepassuser@a.com";

        {
            let data = Data::new().await;
            delete_user(NAME, &data).await;
        }

        let (data, _, signin_resp) = register_and_signin(NAME, EMAIL, PASSWORD).await;
        let cookies = get_cookie!(signin_resp);
        let app = get_app!(data).await;

        let new_password = "newpassword";

        let update_password = ChangePasswordReqest {
            password: PASSWORD.into(),
            new_password: new_password.into(),
            confirm_new_password: PASSWORD.into(),
        };

        let res = update_password_runner(NAME, update_password.into(), &data).await;
        assert!(res.is_err());
        assert_eq!(res, Err(ServiceError::PasswordsDontMatch));

        let update_password = ChangePasswordReqest {
            password: PASSWORD.into(),
            new_password: new_password.into(),
            confirm_new_password: new_password.into(),
        };

        assert!(update_password_runner(NAME, update_password.into(), &data)
            .await
            .is_ok());

        let update_password = ChangePasswordReqest {
            password: new_password.into(),
            new_password: new_password.into(),
            confirm_new_password: PASSWORD.into(),
        };

        bad_post_req_test(
            NAME,
            new_password,
            ROUTES.account.update_password,
            &update_password,
            ServiceError::PasswordsDontMatch,
        )
        .await;

        let update_password = ChangePasswordReqest {
            password: PASSWORD.into(),
            new_password: PASSWORD.into(),
            confirm_new_password: PASSWORD.into(),
        };

        bad_post_req_test(
            NAME,
            new_password,
            ROUTES.account.update_password,
            &update_password,
            ServiceError::WrongPassword,
        )
        .await;

        let update_password = ChangePasswordReqest {
            password: new_password.into(),
            new_password: PASSWORD.into(),
            confirm_new_password: PASSWORD.into(),
        };

        let update_password_resp = test::call_service(
            &app,
            post_request!(&update_password, ROUTES.account.update_password)
                .cookie(cookies)
                .to_request(),
        )
        .await;
        assert_eq!(update_password_resp.status(), StatusCode::OK);
    }
}

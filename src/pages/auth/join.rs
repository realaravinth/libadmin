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
use actix_web::HttpResponseBuilder;
use actix_web::{error::ResponseError, http::header, web, HttpResponse, Responder};
use lazy_static::lazy_static;
use sailfish::TemplateOnce;

use crate::api::v1::auth::runners;
use crate::errors::*;
use crate::pages::errors::ErrorPage;
use crate::AppData;
use crate::PAGES;

#[derive(Clone, TemplateOnce)]
#[template(path = "auth/join/index.html")]
struct IndexPage<'a> {
    error: Option<ErrorPage<'a>>,
}

const PAGE: &str = "Join";

impl<'a> Default for IndexPage<'a> {
    fn default() -> Self {
        IndexPage { error: None }
    }
}

impl<'a> IndexPage<'a> {
    pub fn new(title: &'a str, message: &'a str) -> Self {
        Self {
            error: Some(ErrorPage::new(title, message)),
        }
    }
}

lazy_static! {
    static ref INDEX: String = IndexPage::default().render_once().unwrap();
}

#[my_codegen::get(path = "crate::PAGES.auth.join")]
pub async fn join() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(&*INDEX)
}

#[my_codegen::post(path = "PAGES.auth.join")]
pub async fn join_submit(
    payload: web::Form<runners::Register>,
    data: AppData,
) -> PageResult<impl Responder> {
    let mut payload = payload.into_inner();
    if payload.email.is_some() && payload.email.as_ref().unwrap().is_empty() {
        payload.email = None;
    }

    match runners::register_runner(&payload, &data).await {
        Ok(()) => Ok(HttpResponse::Found()
            .insert_header((header::LOCATION, PAGES.auth.login))
            .finish()),
        Err(e) => {
            let status = e.status_code();
            let heading = status.canonical_reason().unwrap_or("Error");
            Ok(HttpResponseBuilder::new(status)
                .content_type("text/html; charset=utf-8")
                .body(
                    IndexPage::new(heading, &format!("{}", e))
                        .render_once()
                        .unwrap(),
                ))
        }
    }
}

#[cfg(test)]
mod tests {
    use actix_web::test;

    use super::*;

    use crate::api::v1::account::{username::runners::username_exists, AccountCheckPayload};
    use crate::api::v1::auth::runners::Register;
    use crate::data::Data;
    use crate::tests::*;
    use crate::*;
    use actix_web::http::StatusCode;

    #[actix_rt::test]
    async fn auth_join_form_works() {
        let data = Data::new().await;
        const NAME: &str = "testuserformjoin";
        const NAME2: &str = "testuserformjoin2";
        const EMAIL: &str = "testuserformjoin@a.com";
        const PASSWORD: &str = "longpassword";

        let app = get_app!(data).await;

        delete_user(NAME, &data).await;

        // 1. Register with email == None
        let mut msg = Register {
            username: NAME.into(),
            password: PASSWORD.into(),
            confirm_password: PASSWORD.into(),
            email: Some(EMAIL.into()),
        };

        let resp = test::call_service(
            &app,
            post_request!(&msg, PAGES.auth.join, FORM).to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::FOUND);
        let headers = resp.headers();
        assert_eq!(headers.get(header::LOCATION).unwrap(), PAGES.auth.login,);

        let account_check = AccountCheckPayload { val: NAME.into() };
        assert!(
            username_exists(&account_check, &AppData::new(data.clone()))
                .await
                .unwrap()
                .exists
        );

        msg.email = None;
        let resp = test::call_service(
            &app,
            post_request!(&msg, PAGES.auth.join, FORM).to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        msg.email = Some(EMAIL.into());
        msg.username = NAME2.into();
        let resp = test::call_service(
            &app,
            post_request!(&msg, PAGES.auth.join, FORM).to_request(),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}

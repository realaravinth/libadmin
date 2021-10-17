use std::sync::Arc;

//use actix_web::cookie::Cookie;
use actix_web::test;
use actix_web::{dev::ServiceResponse, error::ResponseError, http::StatusCode};
use serde::Serialize;

use super::*;
use crate::api::v1::auth::runners::{Login, Register};
use crate::api::v1::ROUTES;
use crate::data::Data;
use crate::errors::*;

#[macro_export]
macro_rules! get_cookie {
    ($resp:expr) => {
        $resp.response().cookies().next().unwrap().to_owned()
    };
}

/// execute before test execution
pub fn init() {
    use crate::settings::*;
    let mut settings = Settings::default();
    settings.allow_demo = true;
    settings.allow_registration = true;
    crate::init(settings);
}

pub async fn delete_user(name: &str, data: &Arc<Data>) {
    let data = AppData::new(data.clone());
    use crate::api::v1::account::delete::runners::delete_user;
    let r = delete_user(name, &data).await;

    println!("Deleting user: {:?}", &r);
}

#[allow(dead_code, clippy::upper_case_acronyms)]
pub struct FORM;

#[macro_export]
macro_rules! post_request {
    ($uri:expr) => {
        test::TestRequest::post().uri($uri)
    };

    ($serializable:expr, $uri:expr) => {
        test::TestRequest::post()
            .uri($uri)
            .insert_header((actix_web::http::header::CONTENT_TYPE, "application/json"))
            .set_payload(serde_json::to_string($serializable).unwrap())
    };

    ($serializable:expr, $uri:expr, FORM) => {
        test::TestRequest::post().uri($uri).set_form($serializable)
    };
}

#[macro_export]
macro_rules! get_works {
    ($app:expr,$route:expr ) => {
        let list_sitekey_resp =
            test::call_service(&$app, test::TestRequest::get().uri($route).to_request()).await;
        assert_eq!(list_sitekey_resp.status(), StatusCode::OK);
    };
}

#[macro_export]
macro_rules! get_app {
    ("APP") => {
        actix_web::App::new()
            .app_data(crate::get_json_err())
            .wrap(crate::get_identity_service())
            .wrap(actix_web::middleware::NormalizePath::new(
                actix_web::middleware::TrailingSlash::Trim,
            ))
            .configure(crate::services)
    };

    () => {
        test::init_service(get_app!("APP"))
    };
    ($data:expr) => {
        test::init_service(get_app!("APP").app_data(actix_web::web::Data::new($data.clone())))
    };
}

/// register and signin utility
pub async fn register_and_signin(
    name: &str,
    email: &str,
    password: &str,
) -> (Arc<data::Data>, Login, ServiceResponse) {
    register(name, email, password).await;
    signin(name, password).await
}

/// register utility
pub async fn register(name: &str, email: &str, password: &str) {
    let data = Data::new().await;
    let app = get_app!(data).await;

    // 1. Register
    let msg = Register {
        username: name.into(),
        password: password.into(),
        confirm_password: password.into(),
        email: Some(email.into()),
    };
    let resp =
        test::call_service(&app, post_request!(&msg, ROUTES.auth.register).to_request()).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

/// signin util
pub async fn signin(name: &str, password: &str) -> (Arc<Data>, Login, ServiceResponse) {
    let data = Data::new().await;
    let app = get_app!(data.clone()).await;

    // 2. signin
    let creds = Login {
        login: name.into(),
        password: password.into(),
    };
    let signin_resp =
        test::call_service(&app, post_request!(&creds, ROUTES.auth.login).to_request()).await;
    assert_eq!(signin_resp.status(), StatusCode::OK);
    (data, creds, signin_resp)
}

/// pub duplicate test
pub async fn bad_post_req_test<T: Serialize>(
    name: &str,
    password: &str,
    url: &str,
    payload: &T,
    err: ServiceError,
) {
    let (data, _, signin_resp) = signin(name, password).await;
    let cookies = get_cookie!(signin_resp);
    let app = get_app!(data).await;

    let resp = test::call_service(
        &app,
        post_request!(&payload, url)
            .cookie(cookies.clone())
            .to_request(),
    )
    .await;
    assert_eq!(resp.status(), err.status_code());
    let resp_err: ErrorToResponse = test::read_body_json(resp).await;
    //println!("{}", txt.error);
    assert_eq!(resp_err.error, format!("{}", err));
}

/// bad post req test without payload
pub async fn bad_post_req_test_witout_payload(
    name: &str,
    password: &str,
    url: &str,
    err: ServiceError,
) {
    let (data, _, signin_resp) = signin(name, password).await;
    let cookies = get_cookie!(signin_resp);
    let app = get_app!(data).await;

    let resp = test::call_service(
        &app,
        post_request!(url).cookie(cookies.clone()).to_request(),
    )
    .await;
    assert_eq!(resp.status(), err.status_code());
    let resp_err: ErrorToResponse = test::read_body_json(resp).await;
    //println!("{}", txt.error);
    assert_eq!(resp_err.error, format!("{}", err));
}

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

use actix_web::http::StatusCode;
use actix_web::test;

use crate::data::Data;
use crate::*;

use crate::tests::*;

#[actix_rt::test]
async fn protected_routes_work() {
    const NAME: &str = "testuser619";
    const PASSWORD: &str = "longpassword2";
    const EMAIL: &str = "testuser119@a.com2";

    let _post_protected_urls = [
        "/api/v1/account/secret/",
        "/api/v1/account/email/",
        "/api/v1/account/delete",
    ];

    let get_protected_urls = ["/logout"];

    {
        let data = Data::new().await;
        delete_user(NAME, &data).await;
    }

    let (data, _, signin_resp) = register_and_signin(NAME, EMAIL, PASSWORD).await;
    let cookies = get_cookie!(signin_resp);
    let app = get_app!(data).await;

    for url in get_protected_urls.iter() {
        let resp = test::call_service(&app, test::TestRequest::get().uri(url).to_request()).await;
        assert_eq!(resp.status(), StatusCode::FOUND);

        let authenticated_resp = test::call_service(
            &app,
            test::TestRequest::get()
                .uri(url)
                .cookie(cookies.clone())
                .to_request(),
        )
        .await;

        if url == &V1_API_ROUTES.auth.logout {
            assert_eq!(authenticated_resp.status(), StatusCode::FOUND);
        } else {
            assert_eq!(authenticated_resp.status(), StatusCode::OK);
        }
    }
}

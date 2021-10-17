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
use std::time::Duration;

use tokio::spawn;
use tokio::time::sleep;

use crate::api::v1::account::delete::runners::delete_user;
use crate::api::v1::account::{username::runners::username_exists, AccountCheckPayload};
use crate::api::v1::auth::runners::{register_runner, Register};
use crate::*;

use errors::*;

/// Demo username
pub const DEMO_USER: &str = "aaronsw";
/// Demo password
pub const DEMO_PASSWORD: &str = "password";

/// register demo user runner
async fn register_demo_user(data: &AppData) -> ServiceResult<()> {
    let user_exists_payload = AccountCheckPayload {
        val: DEMO_USER.into(),
    };

    if !username_exists(&user_exists_payload, data).await?.exists {
        let register_payload = Register {
            username: DEMO_USER.into(),
            password: DEMO_PASSWORD.into(),
            confirm_password: DEMO_PASSWORD.into(),
            email: None,
        };

        log::info!("Registering demo user");
        match register_runner(&register_payload, data).await {
            Err(ServiceError::UsernameTaken) | Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    } else {
        Ok(())
    }
}

async fn delete_demo_user(data: &AppData) -> ServiceResult<()> {
    log::info!("Deleting demo user");
    delete_user(DEMO_USER, data).await?;
    Ok(())
}

pub async fn run(data: AppData, duration: Duration) -> ServiceResult<()> {
    register_demo_user(&data).await?;

    let fut = async move {
        loop {
            sleep(duration).await;
            if let Err(e) = delete_demo_user(&data).await {
                log::error!("Error while deleting demo user: {:?}", e);
            }
            if let Err(e) = register_demo_user(&data).await {
                log::error!("Error while registering demo user: {:?}", e);
            }
        }
    };
    spawn(fut);
    Ok(())
}

#[cfg(test)]
mod tests {

    use actix_web::test;

    use super::*;
    use crate::tests::*;

    const DURATION: u64 = 5;

    #[actix_rt::test]
    async fn demo_account_works() {
        {
            let data = Data::new().await;
            crate::tests::delete_user(DEMO_USER, &data).await;
        }
        let data = AppData::new(Data::new().await);
        let duration = Duration::from_secs(DURATION);

        // register works
        let _ = register_demo_user(&data).await.unwrap();
        let payload = AccountCheckPayload {
            val: DEMO_USER.into(),
        };
        assert!(username_exists(&payload, &data).await.unwrap().exists);
        signin(DEMO_USER, DEMO_PASSWORD).await;

        // deletion works
        assert!(super::delete_demo_user(&data).await.is_ok());
        assert!(!username_exists(&payload, &data).await.unwrap().exists);

        run(data.clone(), duration).await.unwrap();

        let (_, _, signin_resp) = signin(DEMO_USER, DEMO_PASSWORD).await;
        let cookies = get_cookie!(signin_resp);
        let app = get_app!(Data::new().await).await;

        sleep(Duration::from_secs(2)).await;
        assert!(username_exists(&payload, &data).await.unwrap().exists);
    }
}

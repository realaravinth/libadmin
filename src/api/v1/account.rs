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
use db_core::prelude::*;
use serde::{Deserialize, Serialize};

pub use super::auth;
use crate::api::v1::get_random;
use crate::errors::*;
use crate::Data;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountCheckPayload {
    pub val: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountCheckResp {
    pub exists: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChangePasswordReqest {
    pub password: String,
    pub new_password: String,
    pub confirm_new_password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Secret {
    pub secret: String,
}

impl<T: LibAdminDatabase> Data<T> {
    /// check if email exists on database
    pub async fn email_exists(&self, email: &str) -> ServiceResult<AccountCheckResp> {
        let resp = AccountCheckResp {
            exists: self.db.email_exists(email).await?,
        };

        Ok(resp)
    }

    /// update email
    pub async fn set_email(&self, username: &str, email: &str) -> ServiceResult<()> {
        self.creds.email(email)?;

        let payload = UpdateEmailPayload { username, email };
        self.db.update_email(&payload).await?;
        Ok(())
    }

    pub async fn username_exists(&self, username: &str) -> ServiceResult<AccountCheckResp> {
        let resp = AccountCheckResp {
            exists: self.db.username_exists(username).await?,
        };

        Ok(resp)
    }

    pub async fn set_username(
        &self,
        current_username: &str,
        new_username: &str,
    ) -> ServiceResult<String> {
        let processed_uname = self.creds.username(new_username)?;

        let db_payload = UpdateUsernamePayload {
            old_username: &current_username,
            new_username: &processed_uname,
        };

        self.db.update_username(&db_payload).await?;
        Ok(processed_uname)
    }

    pub async fn get_secret(&self, username: &str) -> ServiceResult<Secret> {
        let secret = Secret {
            secret: self.db.get_secret(username).await?,
        };

        Ok(secret)
    }

    pub async fn update_user_secret(&self, username: &str) -> ServiceResult<String> {
        let mut secret;
        loop {
            secret = get_random(32);

            match self.db.update_secret(&username, &secret).await {
                Ok(_) => break,
                Err(DBError::DuplicateSecret) => continue,
                Err(e) => return Err(e.into()),
            }
        }

        Ok(secret)
    }

    // returns Ok(()) upon successful authentication
    async fn authenticate(&self, username: &str, password: &str) -> ServiceResult<()> {
        use argon2_creds::Config;
        let resp = self.db.username_login(username).await?;
        if Config::verify(&resp.password, password)? {
            Ok(())
        } else {
            Err(ServiceError::WrongPassword)
        }
    }

    pub async fn delete_user(&self, username: &str, password: &str) -> ServiceResult<()> {
        self.authenticate(username, password).await?;
        self.db.delete_account(username).await?;
        Ok(())
    }

    pub async fn update_password(
        &self,
        username: &str,
        payload: &ChangePasswordReqest,
    ) -> ServiceResult<()> {
        if payload.new_password != payload.confirm_new_password {
            return Err(ServiceError::PasswordsDontMatch);
        }

        self.authenticate(username, &payload.password).await?;

        let new_hash = self.creds.password(&payload.new_password)?;

        let db_payload = Creds {
            username: username.into(),
            password: new_hash,
        };

        self.db.update_password(&db_payload).await?;

        Ok(())
    }
}

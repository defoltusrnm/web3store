use std::fmt::Display;

use reqwest::{Client, IntoUrl, Response};
use serde::{Serialize, de::DeserializeOwned};

use super::errors::AppErr;

pub trait ResponseExtended {
    fn ensure_success(self) -> impl Future<Output = Result<(), AppErr>>;
    fn ensure_success_json<T: DeserializeOwned>(self) -> impl Future<Output = Result<T, AppErr>>;
}

impl ResponseExtended for Response {
    async fn ensure_success(self) -> Result<(), AppErr> {
        let status = self.status();

        if status.as_u16() < 400 {
            Ok(())
        } else {
            let url = self.url().to_owned();

            let body = self
                .text()
                .await
                .inspect_err(|err| log::warn!("cannot read body on create user: {err}"))
                .ok()
                .unwrap_or("".to_owned());

            Err(AppErr::from_owned(format!(
                "request {0} failed with {body}",
                url
            )))
        }
    }

    async fn ensure_success_json<T: DeserializeOwned>(self) -> Result<T, AppErr> {
        let status = self.status();

        if status.as_u16() < 400 {
            let payload = self.json::<T>().await.map_err(|err| {
                AppErr::from_owned(format!("failed to parse payload(request succeeded): {err}"))
            })?;

            Ok(payload)
        } else {
            let url = self.url().to_owned();

            let body = self
                .text()
                .await
                .inspect_err(|err| log::warn!("cannot read body on create user: {err}"))
                .ok()
                .unwrap_or("".to_owned());

            Err(AppErr::from_owned(format!(
                "request {0} failed with {body}",
                url
            )))
        }
    }
}

pub trait SendExtended {
    fn quick_post<U: IntoUrl + Display + Copy, T: Display, Body: Serialize + ?Sized>(
        self,
        url: U,
        body: &Body,
        access_token: Option<T>,
    ) -> impl Future<Output = Result<Response, AppErr>>;
    fn quick_get<U: IntoUrl + Display + Copy, T: Display>(
        self,
        url: U,
        access_token: Option<T>,
    ) -> impl Future<Output = Result<Response, AppErr>>;
}

impl SendExtended for Client {
    async fn quick_get<U: IntoUrl + Display + Copy, T: Display>(
        self,
        url: U,
        access_token: Option<T>,
    ) -> Result<Response, AppErr> {
        let method = self.get(url);
        let method_with_auth = match access_token {
            Some(token) => method.bearer_auth(token),
            None => method,
        };

        method_with_auth
            .send()
            .await
            .map_err(|err| AppErr::from_owned(format!("get {0} failed with {err}", url)))
    }

    async fn quick_post<U: IntoUrl + Display + Copy, T: Display, Body: Serialize + ?Sized>(
        self,
        url: U,
        body: &Body,
        access_token: Option<T>,
    ) -> Result<Response, AppErr> {
        let method = self.post(url);
        let method_with_auth = match access_token {
            Some(token) => method.bearer_auth(token),
            None => method,
        };

        method_with_auth
            .json(body)
            .send()
            .await
            .map_err(|err| AppErr::from_owned(format!("post {0} failed with {err}", url)))
    }
}

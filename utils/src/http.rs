use std::fmt::Display;

use futures::TryFutureExt;
use reqwest::{Client, IntoUrl, Response};
use serde::{Serialize, de::DeserializeOwned};

use crate::errors::HttpAppErr;

use super::errors::AppErr;

pub trait ResponseExtended {
    fn ensure_success(self) -> impl Future<Output = Result<(), AppErr>> + Send;
    fn ensure_success_json<T: DeserializeOwned>(
        self,
    ) -> impl Future<Output = Result<T, HttpAppErr>> + Send;
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
                .inspect_err(|err| log::warn!("cannot read body on create user: {err}"))
                .await
                .ok()
                .unwrap_or("".to_owned());

            Err(AppErr::from_owned(format!(
                "request {0} failed with {body}",
                url
            )))
        }
    }

    async fn ensure_success_json<T: DeserializeOwned>(self) -> Result<T, HttpAppErr> {
        let status = self.status();

        if status.as_u16() < 400 {
            let payload = self
                .json::<T>()
                .map_err(|err| HttpAppErr::new(status, &format!("err: {err}")))
                .await?;

            Ok(payload)
        } else {
            let body = self
                .text()
                .inspect_err(|err| log::warn!("cannot read body on create user: {err}"))
                .await
                .ok()
                .unwrap_or("".to_owned());

            Err(HttpAppErr::new(status, &body))
        }
    }
}

pub trait SendExtended {
    fn quick_post(
        self,
        url: impl IntoUrl + Display + Copy + Send,
        body: &(impl Serialize + ?Sized + Send),
        access_token: Option<impl Display + Send>,
    ) -> impl Future<Output = Result<Response, AppErr>>;
    fn quick_put(
        self,
        url: impl IntoUrl + Display + Copy + Send,
        body: &(impl Serialize + ?Sized + Send),
        access_token: Option<impl Display + Send>,
    ) -> impl Future<Output = Result<Response, AppErr>>;
    fn quick_get(
        self,
        url: impl IntoUrl + Display + Copy + Send,
        access_token: Option<impl Display + Send>,
    ) -> impl Future<Output = Result<Response, AppErr>> + Send;
}

impl SendExtended for Client {
    async fn quick_get(
        self,
        url: impl IntoUrl + Display + Copy + Send,
        access_token: Option<impl Display + Send>,
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

    async fn quick_post(
        self,
        url: impl IntoUrl + Display + Copy + Send,
        body: &(impl Serialize + ?Sized + Send),
        access_token: Option<impl Display + Send>,
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

    async fn quick_put(
        self,
        url: impl IntoUrl + Display + Copy + Send,
        body: &(impl Serialize + ?Sized + Send),
        access_token: Option<impl Display + Send>,
    ) -> Result<Response, AppErr> {
        let method = self.put(url);
        let method_with_auth = match access_token {
            Some(token) => method.bearer_auth(token),
            None => method,
        };

        method_with_auth
            .json(body)
            .send()
            .await
            .map_err(|err| AppErr::from_owned(format!("put {0} failed with {err}", url)))
    }
}

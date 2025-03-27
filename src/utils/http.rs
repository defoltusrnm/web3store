use std::fmt::Display;

use reqwest::{Client, IntoUrl, Response};
use serde::Serialize;

use super::errors::AppErr;

pub trait SendExtended {
    fn post_json<U: IntoUrl + Display + Copy, T: Display, Body: Serialize + ?Sized>(
        self,
        url: U,
        body: &Body,
        access_token: Option<T>,
    ) -> impl Future<Output = Result<Response, AppErr>>;
    fn protected_get<U: IntoUrl + Display + Copy, T: Display>(
        self,
        url: U,
        access_token: Option<T>,
    ) -> impl Future<Output = Result<Response, AppErr>>;
}

impl SendExtended for Client {
    async fn protected_get<U: IntoUrl + Display + Copy, T: Display>(
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

    async fn post_json<U: IntoUrl + Display + Copy, T: Display, Body: Serialize + ?Sized>(
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

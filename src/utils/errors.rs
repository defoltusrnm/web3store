use axum::{Json, response::IntoResponse};
use derive_more::Display;
use http::StatusCode;
use serde::Serialize;

#[derive(Display, Debug)]
pub struct AppErr {
    msg: String,
}

impl AppErr {
    pub fn from_owned(msg: String) -> Self {
        AppErr { msg }
    }
    pub fn from(msg: &str) -> Self {
        AppErr {
            msg: msg.to_owned(),
        }
    }
}

pub struct HttpAppErr {
    pub status: StatusCode,
    pub reason: String,
}

impl HttpAppErr {
    pub fn new(status: StatusCode, reason: &str) -> Self {
        HttpAppErr {
            status,
            reason: reason.to_owned(),
        }
    }

    pub fn from(status: StatusCode, app_err: AppErr) -> Self {
        HttpAppErr {
            status,
            reason: app_err.msg,
        }
    }
}

impl IntoResponse for HttpAppErr {
    fn into_response(self) -> axum::response::Response {
        let error_msg = HttpErrorMessage {
            title: self.status.as_str().to_owned(),
            status: self.status.as_u16(),
            reason: self.reason,
        };

        (self.status, Json(error_msg)).into_response()
    }
}

#[derive(Serialize)]
struct HttpErrorMessage {
    pub title: String,
    pub status: u16,
    pub reason: String,
}

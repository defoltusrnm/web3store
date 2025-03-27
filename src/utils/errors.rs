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
    pub fn cancelled<T>() -> Result<T, AppErr> {
        Result::<T, AppErr>::Err(AppErr::from("op cancelled"))
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

    pub fn failed_dependency(err: AppErr) -> Self {
        HttpAppErr {
            status: StatusCode::FAILED_DEPENDENCY,
            reason: err.msg,
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

impl IntoResponse for AppErr {
    fn into_response(self) -> axum::response::Response {
        let error_msg = HttpErrorMessage {
            title: StatusCode::INTERNAL_SERVER_ERROR.as_str().to_owned(),
            status: 500,
            reason: "internal error".to_owned(),
        };

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_msg)).into_response()
    }
}

#[derive(Serialize)]
struct HttpErrorMessage {
    pub title: String,
    pub status: u16,
    pub reason: String,
}

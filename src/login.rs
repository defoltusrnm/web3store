use std::collections::HashMap;

use axum::{
    Json, Router,
    response::{IntoResponse, Result},
    routing::post,
};
use http::StatusCode;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    keycloak::services::{
        host_implementation::EnvHostAddressProvider, routes::Routes,
        routes_implementation::DefaultRoutes,
    },
    utils::{env::env_var, errors::HttpAppErr},
};

pub fn create_login_router() -> Router {
    Router::new().route("/api/login", post(login))
}

async fn login(Json(request): Json<LoginRequest>) -> Result<LoginResponse> {
    let host_provider = &EnvHostAddressProvider::new("KEYCLOAK_HOST");
    let routes = &DefaultRoutes::new(host_provider);

    let realm_name = env_var("KEYCLOAK_REALM")
        .map_err(|_| HttpAppErr::new(StatusCode::INTERNAL_SERVER_ERROR, "Error"))?;
    let client_name = env_var("KEYCLOAK_CLIENT")
        .map_err(|_| HttpAppErr::new(StatusCode::INTERNAL_SERVER_ERROR, "Error"))?;

    let auth_url = routes
        .get_auth_route(&realm_name)
        .await
        .inspect_err(|err| log::error!("could not get url {err}"))
        .map_err(|_| HttpAppErr::new(StatusCode::INTERNAL_SERVER_ERROR, ""))?;

    let mut params = HashMap::new();
    params.insert("client_id", client_name);
    params.insert("username", request.login);
    params.insert("password", request.password);
    params.insert("grant_type", "password".to_owned());

    let response = Client::new()
        .post(auth_url)
        .form(&params)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await
        .inspect_err(|err| log::error!("auth err: {err}"))
        .map_err(|_| HttpAppErr::new(StatusCode::FAILED_DEPENDENCY, "keycloak failed"))?;

    let status_code = response.status();

    let res = match status_code {
        StatusCode::OK => response
            .json::<LoginResponse>()
            .await
            .inspect_err(|err| log::error!("error reading response: {err}"))
            .map_err(|_| HttpAppErr::new(StatusCode::FAILED_DEPENDENCY, "keycloak_error")),
        _ => {
            let body = response
                .text()
                .await
                .inspect_err(|err| log::error!("failed to read body: {err}"))
                .ok()
                .unwrap_or("".to_owned());
            Result::<LoginResponse, HttpAppErr>::Err(HttpAppErr::new(status_code, &body))
        }
    }?;

    Ok(res)
}

#[derive(Deserialize)]
struct LoginRequest {
    pub login: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
}

impl IntoResponse for LoginResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

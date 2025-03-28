use std::collections::HashMap;

use axum::{
    Json, Router,
    response::{self, IntoResponse},
    routing::post,
};
use futures::TryFutureExt;
use http::StatusCode;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use utils::{env::env_var, errors::HttpAppErr, http::ResponseExtended};

use crate::keycloak::{
    keycloak_ex::KeycloakExtensions,
    services::{
        host_implementation::EnvHostAddressProvider, routes::Routes,
        routes_implementation::DefaultRoutes,
    },
};

pub fn create_refresh_token_router() -> Router {
    Router::new().route("/api/token", post(refresh_token))
}

async fn refresh_token(Json(request): Json<LoginRequest>) -> response::Result<LoginResponse> {
    let host_provider = &EnvHostAddressProvider::new("KEYCLOAK_HOST");
    let routes = &DefaultRoutes::new(host_provider);

    let auth_url = routes
        .get_auth_route(&env_var("KEYCLOAK_REALM")?)
        .log_err()
        .await?;

    let mut params = HashMap::new();
    params.insert("client_id", env_var("KEYCLOAK_CLIENT")?);
    params.insert("refresh_token", request.refresh_token);
    params.insert("grant_type", "refresh_token".to_owned());

    let response = Client::new()
        .post(auth_url)
        .form(&params)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .inspect_err(|err| log::error!("auth err: {err}"))
        .map_err(|_| HttpAppErr::new(StatusCode::FAILED_DEPENDENCY, "keycloak failed"))
        .await?;

    let res = response
        .ensure_success_json::<LoginResponse>()
        .log_err()
        .await?;

    Ok(res)
}

#[derive(Deserialize)]
struct LoginRequest {
    pub refresh_token: String,
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

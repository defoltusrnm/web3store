use axum::{Json, Router, response::Result, routing::post};
use http::StatusCode;
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

    let auth_url = routes.get_auth_route(&realm_name).await;

    Ok(LoginResponse {
        access_token: "123".to_owned(),
        refresh_token: "123".to_owned(),
    })
}

#[derive(Deserialize)]
struct LoginRequest {
    pub login: String,
    pub password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
}

use axum::{routing::post, Json, Router};
use http::StatusCode;
use serde::Deserialize;
use tokio_util::sync::CancellationToken;

use crate::{keycloak::{keycloak_factory::create_default_manager, requests::create_user::CreateUserRequest}, utils::env::env_var};

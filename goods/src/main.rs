use axum::Router;
use futures::TryFutureExt;
use utils::{dotenv::configure_dotenv, env::env_var, errors::AppErr, logging::configure_logs};

#[tokio::main]
async fn main() -> Result<(), AppErr> {
    configure_dotenv();
    _ = configure_logs(log::LevelFilter::Info)?;

    let app = Router::new();

    let listener = tokio::net::TcpListener::bind(env_var("SERVICE_HOST")?)
        .map_err(|err| AppErr::from_owned(format!("failed to bind: {err}")))
        .await?;

    log::info!("app started at: {0}", env_var("SERVICE_HOST")?);

    axum::serve(listener, app)
        .await
        .map_err(|err| AppErr::from_owned(format!("server failed {err}")))?;

    Ok(())
}

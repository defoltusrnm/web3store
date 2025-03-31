use std::time::Duration;

use futures::TryFutureExt;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use utils::{env::env_var, errors::AppErr};

pub async fn get_db_conn() -> Result<DatabaseConnection, AppErr> {
    let mut opt = ConnectOptions::new(env_var("DB_HOST")?);
    opt.max_connections(100)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .connect_lazy(true);

    let db = Database::connect(opt)
        .map_err(|err| AppErr::from_owned(format!("failed to connect: {err}")))
        .await?;

    Ok(db)
}

pub mod create_customer_topic;
pub mod db_factory;
pub mod entity;
pub mod migrations;

use create_customer_topic::CreateCustomerTopic;
use db_factory::get_db_conn;
use futures::TryFutureExt;
use sea_orm_migration::MigratorTrait;
use utils::{
    dotenv::configure_dotenv,
    errors::AppErr,
    kafka_consumer::{self},
    logging::configure_logs,
};

#[tokio::main]
async fn main() -> Result<(), AppErr> {
    configure_dotenv();
    _ = configure_logs(log::LevelFilter::Debug)?;

    let db = get_db_conn().await?;
    migrations::migrator::Migrator::up(&db, None)
        .map_err(|err| AppErr::from_owned(format!("failed to migrate database: {err}")))
        .await?;

    kafka_consumer::consume_topic::<CreateCustomerTopic>().await?;

    Ok(())
}

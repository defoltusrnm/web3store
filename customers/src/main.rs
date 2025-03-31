pub mod entity;
pub mod migrations;

use futures::{StreamExt, TryFutureExt};
use rdkafka::{
    ClientConfig, Message,
    consumer::{Consumer, StreamConsumer},
};
use sea_orm::Database;
use sea_orm_migration::MigratorTrait;
use utils::{dotenv::configure_dotenv, env::env_var, errors::AppErr, logging::configure_logs};

#[tokio::main]
async fn main() -> Result<(), AppErr> {
    configure_dotenv();
    _ = configure_logs(log::LevelFilter::Info)?;

    let db = Database::connect(env_var("POSTGRES_HOST")?)
        .map_err(|err| AppErr::from_owned(format!("failed to connect to db: {err}")))
        .await?;

    migrations::migrator::Migrator::up(&db, None)
        .map_err(|err| AppErr::from_owned(format!("failed to migrate database: {err}")))
        .await?;

    consume_topic().await?;

    Ok(())
}

async fn consume_topic() -> Result<(), AppErr> {
    let kafka_host = env_var("KAFKA_HOST")?;
    let kafka_topic = &env_var("KAFKA_CUSTOMER_TOPIC")?;

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", kafka_host)
        .set("group.id", "solution_group")
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "earliest")
        .create()
        .map_err(|err| AppErr::from_owned(format!("failed to start consumer: {err}")))?;

    consumer
        .subscribe(&[kafka_topic])
        .map_err(|err| AppErr::from_owned(format!("failed to start consumer: {err}")))?;

    while let Some(message) = consumer.stream().next().await {
        match message {
            Ok(m) => {
                if let Some(payload) = m.payload() {
                    log::info!("Received message: {}", String::from_utf8_lossy(payload));
                }
            }
            Err(e) => log::error!("Kafka error: {}", e),
        }
    }

    Ok(())
}

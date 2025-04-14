use std::time::Duration;

use futures::TryFutureExt;
use rdkafka::{
    ClientConfig,
    producer::{FutureProducer, FutureRecord},
};
use serde::Serialize;
use utils::errors::AppErr;

pub async fn produce_event<T: Serialize>(
    host: &str,
    topic: &str,
    message: T,
) -> Result<(), AppErr> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", host)
        .create()
        .map_err(|err| AppErr::from_owned(format!("failed to create kafka client {err}")))?;

    let payload = serde_json::to_string(&message)
        .map_err(|err| AppErr::from_owned(format!("failed to serialize payload: {err}")))?;

    let record: FutureRecord<'_, String, String> = FutureRecord::to(topic).payload(&payload);

    let _delivery = producer
        .send(record, Duration::from_secs(1))
        .map_err(|err| AppErr::from_owned(format!("failed to produce message: {0}", err.0)))
        .await?;

    Ok(())
}

use futures::StreamExt;
use rdkafka::{
    ClientConfig, Message,
    consumer::{Consumer, StreamConsumer},
};

use crate::errors::AppErr;

pub trait KafkaTopic {
    fn get_descriptor() -> Result<KafkaTopicDescriptor, AppErr>;
    fn handle_message(payload: &[u8]) -> impl Future<Output = Result<(), AppErr>>;
}

pub struct KafkaTopicDescriptor {
    pub host: String,
    pub topic: String,
}

pub async fn consume_topic<Topic: KafkaTopic>() -> Result<(), AppErr> {
    let descriptor = Topic::get_descriptor()?;

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", descriptor.host)
        .set("group.id", "solution_group")
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "earliest")
        .create()
        .map_err(|err| AppErr::from_owned(format!("failed to start consumer: {err}")))?;

    consumer
        .subscribe(&[&descriptor.topic])
        .map_err(|err| AppErr::from_owned(format!("failed to start consumer: {err}")))?;

    while let Some(message) = consumer.stream().next().await {
        match message {
            Ok(m) => {
                if let Some(payload) = m.payload() {
                    match Topic::handle_message(payload).await {
                        Ok(_) => log::debug!("message handled"),
                        Err(err) => log::error!("failed message handling: {err}"),
                    }
                }
            }
            Err(e) => log::error!("Kafka error: {}", e),
        }
    }

    Ok(())
}

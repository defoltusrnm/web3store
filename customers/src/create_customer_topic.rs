use futures::TryFutureExt;
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use serde::Deserialize;
use utils::{
    env::env_var,
    errors::AppErr,
    kafka_consumer::{KafkaTopic, KafkaTopicDescriptor},
};

use crate::db_factory::get_db_conn;

pub struct CreateCustomerTopic;
impl KafkaTopic for CreateCustomerTopic {
    fn get_descriptor() -> Result<utils::kafka_consumer::KafkaTopicDescriptor, AppErr> {
        let descriptor = KafkaTopicDescriptor {
            host: env_var("KAFKA_HOST")?,
            topic: env_var("KAFKA_CUSTOMER_TOPIC")?,
        };

        Ok(descriptor)
    }

    async fn handle_message(payload: &[u8]) -> Result<(), AppErr> {
        let event = serde_json::from_slice::<CreateCustomerEvent>(payload)
            .map_err(|err| AppErr::from_owned(format!("failed at serialization: {err}")))?;

        let customer = crate::entity::customer::ActiveModel {
            email: Set(event.email),
            ..Default::default()
        };

        let db = get_db_conn().await?;
        customer
            .insert(&db)
            .map_err(|err| AppErr::from_owned(format!("failed to create customer: {err}")))
            .await?;

        log::info!("new customer created");
        Ok(())
    }
}

#[derive(Deserialize)]
struct CreateCustomerEvent {
    pub email: String,
}

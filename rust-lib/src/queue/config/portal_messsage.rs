use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions, BasicConsumeOptions},
    types::FieldTable,
    BasicProperties,
};

use super::{ConsumeConfig, PublishConfig, QueueConfig};

const NAME: &str = "portal_message";
pub fn queue_portal_message() -> QueueConfig {
    QueueConfig {
        name: NAME.to_string(),
        options: QueueDeclareOptions::default(),
        field_table: FieldTable::default(),
    }
}

pub fn publish_portal_message() -> PublishConfig {
    PublishConfig {
        name: NAME.to_string(),
        exchange: "".to_string(),
        options: BasicPublishOptions::default(),
        properties: BasicProperties::default(),
    }
}

pub fn consume_portal_message() -> ConsumeConfig {
    ConsumeConfig {
        name: NAME.to_string(),
        options: BasicConsumeOptions::default(),
        field_table: FieldTable::default(),
    }
}

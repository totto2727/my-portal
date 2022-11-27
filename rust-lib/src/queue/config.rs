pub mod portal_messsage;

use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions, BasicConsumeOptions},
    types::FieldTable,
    BasicProperties,
};

#[derive(Debug)]
pub struct QueueConfig {
    pub name: String,
    pub options: QueueDeclareOptions,
    pub field_table: FieldTable,
}

#[derive(Debug)]
pub struct PublishConfig {
    pub name: String,
    pub exchange: String,
    pub options: BasicPublishOptions,
    pub properties: BasicProperties,
}

#[derive(Debug)]
pub struct ConsumeConfig {
    pub name: String,
    pub options: BasicConsumeOptions,
    pub field_table: FieldTable,
}

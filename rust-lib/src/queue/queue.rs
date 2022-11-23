use std::error::Error;

use deadpool::managed::Pool;
use deadpool_lapin::Manager;
use lapin::Channel;
use serde::Serialize;

use super::{
    config::{
        portal_messsage::{publish_portal_message, queue_portal_message},
        PublishConfig,
    },
    QueueConfig,
};

pub enum Queue {
    PortalMessage,
}
impl Queue {
    pub fn get_queue_config(self) -> QueueConfig {
        match self {
            Queue::PortalMessage => queue_portal_message(),
        }
    }

    pub fn get_publish_config(self) -> PublishConfig {
        match self {
            Queue::PortalMessage => publish_portal_message(),
        }
    }

    pub async fn create_channel(pool: &Pool<Manager>) -> Result<Channel, Box<dyn Error>> {
        Ok(pool.get().await?.create_channel().await?)
    }

    pub async fn declare(self, channel: &Channel) -> Result<lapin::Queue, lapin::Error> {
        let config = self.get_queue_config();
        channel
            .queue_declare(config.name.as_str(), config.options, config.field_table)
            .await
    }

    pub async fn publish<T: ?Sized + Serialize>(
        self,
        channel: &Channel,
        payload: &T,
    ) -> Result<lapin::publisher_confirm::PublisherConfirm, Box<dyn Error>> {
        let config = self.get_publish_config();
        let json = serde_json::to_vec(&payload)?;
        Ok(channel
            .basic_publish(
                &config.exchange,
                &config.name,
                config.options,
                &json,
                config.properties,
            )
            .await?)
    }
}

pub use sea_orm_migration::prelude::*;

mod m20221203_185943_create_tag;
mod m20221203_202947_create_source_platform;
mod m20221203_203001_create_portal_platform;
mod m20221204_012419_create_rule;
mod m20221205_220619_create_tagged_rule;
mod m20221205_230421_create_channel;
mod m20221205_230434_create_tagged_channel;
mod m20221206_000136_create_user;
mod m20221206_000154_create_tagged_user;
mod m20221206_125718_create_message;
mod m20221206_125730_create_tagged_message;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221203_185943_create_tag::Migration),
            Box::new(m20221203_202947_create_source_platform::Migration),
            Box::new(m20221203_203001_create_portal_platform::Migration),
            Box::new(m20221204_012419_create_rule::Migration),
            Box::new(m20221205_220619_create_tagged_rule::Migration),
            Box::new(m20221205_230421_create_channel::Migration),
            Box::new(m20221205_230434_create_tagged_channel::Migration),
            Box::new(m20221206_000136_create_user::Migration),
            Box::new(m20221206_000154_create_tagged_user::Migration),
            Box::new(m20221206_125718_create_message::Migration),
            Box::new(m20221206_125730_create_tagged_message::Migration),
        ]
    }
}

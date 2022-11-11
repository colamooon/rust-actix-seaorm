pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20221110_104334_create_post;
mod m20221110_110152_create_member_info;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20221110_104334_create_post::Migration),
            Box::new(m20221110_110152_create_member_info::Migration),
        ]
    }
}

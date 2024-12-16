pub use sea_orm_migration::prelude::*;

mod m20241211_185925_user;
mod m20241211_190006_file;
mod m20241211_190013_invite;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241211_185925_user::Migration),
            Box::new(m20241211_190006_file::Migration),
            Box::new(m20241211_190013_invite::Migration),
        ]
    }
}
